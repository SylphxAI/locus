use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use regex::Regex;
use walkdir::WalkDir;

use crate::store::{
    hash_file_bytes, load_file_hashes, load_index, save_file_hashes, save_index, FileHashManifest,
};
use crate::tokenize::{tokenize, unique_terms};
use crate::types::{IndexStats, ScoreComponent};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Chunk {
    pub path: String,
    pub start_line: u32,
    pub end_line: u32,
    pub text: String,
    pub tokens: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol_name: Option<String>,
    #[serde(default = "default_chunk_type")]
    pub chunk_type: String,
}

fn default_chunk_type() -> String {
    "file".into()
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchIndex {
    pub root: String,
    pub chunks: Vec<Chunk>,
    pub doc_freq: HashMap<String, usize>,
    pub avg_doc_len: f64,
}

const DEFAULT_EXCLUDES: &[&str] = &["node_modules", "dist", "target", ".git"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexMode {
    Full,
    Auto,
}

impl IndexMode {
    pub fn parse(value: &str) -> Self {
        match value {
            "full" => Self::Full,
            _ => Self::Auto,
        }
    }
}

pub fn refresh_index(
    root: &Path,
    max_file_bytes: u64,
    mode: IndexMode,
) -> Result<(SearchIndex, IndexStats), String> {
    if mode == IndexMode::Full {
        return build_index(root, max_file_bytes);
    }

    let started = Instant::now();
    let canonical = root
        .canonicalize()
        .map_err(|e| format!("INVALID_ROOT: {e}"))?;
    let inventory = inventory_files(&canonical, max_file_bytes)?;

    if let (Ok(index), Ok(stored)) = (load_index(&canonical), load_file_hashes(&canonical)) {
        if stored.file_hashes == inventory {
            return Ok((
                index.clone(),
                IndexStats {
                    files_scanned: inventory.len(),
                    chunks_indexed: index.chunks.len(),
                    elapsed_ms: started.elapsed().as_millis() as u64,
                    refresh_mode: "cache_hit".into(),
                    files_changed: 0,
                    files_removed: 0,
                },
            ));
        }

        let mut changed = HashSet::new();
        let mut removed = HashSet::new();

        for (path, hash) in &inventory {
            match stored.file_hashes.get(path) {
                Some(previous) if previous == hash => {}
                _ => {
                    changed.insert(path.clone());
                }
            }
        }

        for path in stored.file_hashes.keys() {
            if !inventory.contains_key(path) {
                removed.insert(path.clone());
            }
        }

        let files_changed = changed.len();
        let files_removed = removed.len();
        let affected: HashSet<String> = changed.union(&removed).cloned().collect();
        let mut index = index;
        index
            .chunks
            .retain(|chunk| !affected.contains(&chunk.path));

        for path in &changed {
            let file_path = canonical.join(path);
            let content = fs::read_to_string(&file_path).unwrap_or_default();
            index.chunks.extend(chunk_file(path, &content));
        }

        rebuild_doc_freq(&mut index);
        let chunks_indexed = index.chunks.len();

        let manifest = FileHashManifest {
            schema_version: crate::store::FILE_HASH_SCHEMA_VERSION.into(),
            root: canonical.to_string_lossy().to_string(),
            file_hashes: inventory.clone(),
        };
        save_index(&canonical, &index)?;
        save_file_hashes(&canonical, &manifest)?;

        return Ok((
            index,
            IndexStats {
                files_scanned: inventory.len(),
                chunks_indexed,
                elapsed_ms: started.elapsed().as_millis() as u64,
                refresh_mode: "incremental".into(),
                files_changed,
                files_removed,
            },
        ));
    }

    build_index(root, max_file_bytes)
}

pub fn inventory_files(root: &Path, max_file_bytes: u64) -> Result<HashMap<String, String>, String> {
    let mut inventory = HashMap::new();

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        let rel = path.strip_prefix(root).unwrap_or(path);
        let rel_str = rel.to_string_lossy().replace('\\', "/");
        if should_skip(&rel_str) || !is_indexable_extension(&rel_str) {
            continue;
        }
        if let Ok(meta) = fs::metadata(path) {
            if meta.len() > max_file_bytes {
                continue;
            }
        }
        let hash = hash_file_bytes(path)?;
        inventory.insert(rel_str, hash);
    }

    Ok(inventory)
}

fn is_indexable_extension(rel: &str) -> bool {
    matches!(
        final_extension(rel),
        Some("ts" | "tsx" | "js" | "jsx" | "mjs" | "cjs" | "rs" | "md" | "py" | "go")
    )
}

/// Final extension only: `file.tsx` is `tsx`, `file.sh` is `sh`, `file.tar.ts` is `ts`.
pub fn final_extension(path: &str) -> Option<&str> {
    let file_name = path.rsplit(['/', '\\']).next().unwrap_or(path);
    let (stem, ext) = file_name.rsplit_once('.')?;
    if stem.is_empty() || ext.is_empty() {
        return None;
    }
    Some(ext)
}

fn rebuild_doc_freq(index: &mut SearchIndex) {
    index.doc_freq.clear();
    let mut total_len = 0usize;
    for chunk in &index.chunks {
        total_len += chunk.tokens.len();
        for term in unique_terms(&chunk.tokens) {
            *index.doc_freq.entry(term).or_insert(0) += 1;
        }
    }
    index.avg_doc_len = if index.chunks.is_empty() {
        0.0
    } else {
        total_len as f64 / index.chunks.len() as f64
    };
}

pub fn build_index(root: &Path, max_file_bytes: u64) -> Result<(SearchIndex, IndexStats), String> {
    let started = Instant::now();
    let root = root
        .canonicalize()
        .map_err(|e| format!("INVALID_ROOT: {e}"))?;
    let mut index = SearchIndex {
        root: root.to_string_lossy().to_string(),
        ..Default::default()
    };
    let mut files_scanned = 0usize;

    for entry in WalkDir::new(&root).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        let rel = path.strip_prefix(&root).unwrap_or(path);
        let rel_str = rel.to_string_lossy().replace('\\', "/");
        if should_skip(&rel_str) {
            continue;
        }
        if !is_indexable_extension(&rel_str) {
            continue;
        }
        if let Ok(meta) = fs::metadata(path) {
            if meta.len() > max_file_bytes {
                continue;
            }
        }
        files_scanned += 1;
        let content = fs::read_to_string(path).unwrap_or_default();
        index.chunks.extend(chunk_file(&rel_str, &content));
    }

    let mut total_len = 0usize;
    for chunk in &index.chunks {
        total_len += chunk.tokens.len();
        for term in unique_terms(&chunk.tokens) {
            *index.doc_freq.entry(term).or_insert(0) += 1;
        }
    }
    index.avg_doc_len = if index.chunks.is_empty() {
        0.0
    } else {
        total_len as f64 / index.chunks.len() as f64
    };

    let chunks_indexed = index.chunks.len();
    let inventory = inventory_files(&root, max_file_bytes)?;
    let manifest = FileHashManifest {
        schema_version: crate::store::FILE_HASH_SCHEMA_VERSION.into(),
        root: root.to_string_lossy().to_string(),
        file_hashes: inventory,
    };
    save_index(&root, &index)?;
    save_file_hashes(&root, &manifest)?;

    Ok((
        index,
        IndexStats {
            files_scanned,
            chunks_indexed,
            elapsed_ms: started.elapsed().as_millis() as u64,
            refresh_mode: "full".into(),
            files_changed: files_scanned,
            files_removed: 0,
        },
    ))
}

fn should_skip(rel: &str) -> bool {
    rel.split('/').any(|part| DEFAULT_EXCLUDES.contains(&part))
}

#[derive(Debug, Clone)]
struct SymbolSpan {
    name: String,
    chunk_type: String,
    start_line: u32,
}

fn chunk_file(path: &str, content: &str) -> Vec<Chunk> {
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len().max(1) as u32;
    let spans = extract_symbol_spans(path, content);

    if spans.is_empty() {
        let path_context = path.replace(['/', '.', '-'], " ");
        return vec![Chunk {
            path: path.into(),
            start_line: 1,
            end_line: total_lines,
            text: content.to_string(),
            tokens: tokenize(&format!("{path_context} {content}")),
            symbol_name: None,
            chunk_type: "file".into(),
        }];
    }

    let preamble: String = lines
        .iter()
        .take(spans.first().map(|span| span.start_line as usize).unwrap_or(1).saturating_sub(1))
        .copied()
        .collect::<Vec<_>>()
        .join("\n");
    let path_context = path.replace(['/', '.', '-'], " ");

    spans
        .iter()
        .enumerate()
        .map(|(index, span)| {
            let end_line = spans
                .get(index + 1)
                .map(|next| next.start_line.saturating_sub(1))
                .unwrap_or(total_lines)
                .max(span.start_line);
            let snippet: String = lines
                .get((span.start_line as usize).saturating_sub(1)..end_line as usize)
                .unwrap_or(&[])
                .join("\n");
            let chunk_text = if preamble.is_empty() {
                snippet.clone()
            } else {
                format!("{preamble}\n{snippet}")
            };
            Chunk {
                path: path.into(),
                start_line: span.start_line,
                end_line,
                text: chunk_text.clone(),
                tokens: tokenize(&format!("{path_context} {} {chunk_text}", span.name)),
                symbol_name: Some(span.name.clone()),
                chunk_type: span.chunk_type.clone(),
            }
        })
        .collect()
}

fn extract_symbol_spans(path: &str, content: &str) -> Vec<SymbolSpan> {
    let mut patterns = vec![
        (
            Regex::new(r"(?m)^export\s+(?:async\s+)?function\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap(),
            "function",
        ),
        (
            Regex::new(r"(?m)^export\s+class\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap(),
            "class",
        ),
        (
            Regex::new(r"(?m)^export\s+const\s+([A-Za-z_][A-Za-z0-9_]*)\s*=").unwrap(),
            "const",
        ),
        (
            Regex::new(r"(?m)^(?:async\s+)?function\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap(),
            "function",
        ),
        (
            Regex::new(r"(?m)^def\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(").unwrap(),
            "function",
        ),
        (
            Regex::new(r"(?m)^class\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap(),
            "class",
        ),
    ];

    match final_extension(path) {
        Some("rs") => {
            patterns.extend([
                (
                    Regex::new(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap(),
                    "function",
                ),
                (
                    Regex::new(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?struct\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap(),
                    "struct",
                ),
                (
                    Regex::new(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?enum\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap(),
                    "enum",
                ),
                (
                    Regex::new(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?trait\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap(),
                    "trait",
                ),
            ]);
        }
        Some("go") => {
            patterns.push((
                Regex::new(r"(?m)^\s*func\s+(?:\([^)\n]*\)\s*)?([A-Za-z_][A-Za-z0-9_]*)").unwrap(),
                "function",
            ));
        }
        _ => {}
    }

    let mut spans = Vec::new();
    for (pattern, chunk_type) in patterns {
        for cap in pattern.captures_iter(content) {
            let name = cap[1].to_string();
            let start_byte = cap.get(0).map(|m| m.start()).unwrap_or(0);
            let start_line = content[..start_byte].matches('\n').count() as u32 + 1;
            spans.push(SymbolSpan {
                name,
                chunk_type: chunk_type.into(),
                start_line,
            });
        }
    }

    spans.sort_by_key(|span| span.start_line);
    spans.dedup_by(|left, right| left.start_line == right.start_line && left.name == right.name);
    spans
}

pub fn find_related_index(
    index: &SearchIndex,
    path: &str,
    line: u32,
    limit: usize,
) -> Vec<crate::types::SearchHit> {
    let Some(seed) = index.chunks.iter().find(|chunk| {
        chunk.path == path && chunk.start_line <= line && line <= chunk.end_line
    }) else {
        return Vec::new();
    };
    let seed_terms: std::collections::HashSet<&str> = seed.tokens.iter().map(String::as_str).collect();
    let mut scored: Vec<(f64, &Chunk)> = index
        .chunks
        .iter()
        .filter(|chunk| !std::ptr::eq(*chunk, seed))
        .map(|chunk| {
            let overlap = chunk
                .tokens
                .iter()
                .filter(|token| seed_terms.contains(token.as_str()))
                .count();
            let union = seed_terms
                .iter()
                .copied()
                .chain(chunk.tokens.iter().map(String::as_str))
                .collect::<std::collections::HashSet<_>>()
                .len();
            let score = if union == 0 { 0.0 } else { overlap as f64 / union as f64 };
            (score, chunk)
        })
        .filter(|(score, _)| *score > 0.0)
        .collect();
    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal).then_with(|| a.1.path.cmp(&b.1.path)));
    scored
        .into_iter()
        .take(limit.max(1))
        .map(|(score, chunk)| crate::types::SearchHit {
            path: chunk.path.clone(),
            score,
            matched_terms: chunk
                .tokens
                .iter()
                .filter(|token| seed_terms.contains(token.as_str()))
                .take(12)
                .cloned()
                .collect(),
            score_components: Vec::new(),
            start_line: Some(chunk.start_line),
            end_line: Some(chunk.end_line),
            snippet: Some(chunk.text.chars().take(360).collect()),
            symbol_name: chunk.symbol_name.clone(),
            chunk_type: Some(chunk.chunk_type.clone()),
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub limit: usize,
    pub include_content: bool,
    pub file_extensions: Vec<String>,
    pub path_filter: Option<String>,
    pub exclude_paths: Vec<String>,
}

impl SearchOptions {
    pub fn limit_only(limit: usize) -> Self {
        Self {
            limit,
            include_content: true,
            file_extensions: Vec::new(),
            path_filter: None,
            exclude_paths: Vec::new(),
        }
    }
}

pub fn search_index(index: &SearchIndex, query: &str, limit: usize) -> Vec<crate::types::SearchHit> {
    search_index_with(index, query, &SearchOptions::limit_only(limit))
}

pub fn search_index_with(
    index: &SearchIndex,
    query: &str,
    options: &SearchOptions,
) -> Vec<crate::types::SearchHit> {
    let query_terms = tokenize(query);
    if query_terms.is_empty() || index.chunks.is_empty() {
        return vec![];
    }

    // Document frequency stays the full index. Filters only decide which hits return.
    let n_docs = index.chunks.len() as f64;
    let k1 = 1.2;
    let b = 0.75;

    let mut scored = Vec::new();
    for chunk in &index.chunks {
        if !chunk_matches_filters(chunk, options) {
            continue;
        }
        let mut score = 0.0;
        let mut matched = Vec::new();
        let doc_len = chunk.tokens.len() as f64;
        let mut term_freq: HashMap<String, usize> = HashMap::new();
        for token in &chunk.tokens {
            *term_freq.entry(token.clone()).or_insert(0) += 1;
        }

        let mut score_components = Vec::new();
        for term in &query_terms {
            let tf = *term_freq.get(term).unwrap_or(&0) as f64;
            if tf == 0.0 {
                continue;
            }
            matched.push(term.clone());
            let df = *index.doc_freq.get(term).unwrap_or(&1) as f64;
            let idf = ((n_docs - df + 0.5) / (df + 0.5) + 1.0).ln();
            let numerator = tf * (k1 + 1.0);
            let denominator = tf + k1 * (1.0 - b + b * (doc_len / index.avg_doc_len.max(1.0)));
            let bm25 = idf * (numerator / denominator);
            score += bm25;
            score_components.push(ScoreComponent {
                term: term.clone(),
                term_frequency: tf,
                document_frequency: df,
                idf,
                bm25,
            });
        }

        if score > 0.0 {
            scored.push(crate::types::SearchHit {
                path: chunk.path.clone(),
                score,
                matched_terms: matched,
                score_components,
                start_line: Some(chunk.start_line),
                end_line: Some(chunk.end_line),
                snippet: if options.include_content {
                    Some(chunk.text.clone())
                } else {
                    None
                },
                symbol_name: chunk.symbol_name.clone(),
                chunk_type: Some(chunk.chunk_type.clone()),
            });
        }
    }

    scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(options.limit);
    scored
}

fn chunk_matches_filters(chunk: &Chunk, options: &SearchOptions) -> bool {
    let path = crate::search_input::normalize_slashes(&chunk.path);
    if let Some(filter) = &options.path_filter {
        let filter = crate::search_input::normalize_slashes(filter);
        if !path.contains(&filter) {
            return false;
        }
    }
    if options.exclude_paths.iter().any(|excluded| {
        let excluded = crate::search_input::normalize_slashes(excluded);
        path.contains(&excluded)
    }) {
        return false;
    }
    if options.file_extensions.is_empty() {
        return true;
    }
    let Some(ext) = final_extension(&path) else {
        return false;
    };
    options.file_extensions.iter().any(|wanted| {
        crate::search_input::extension_token(wanted).as_deref() == Some(ext)
    })
}

pub fn index_path(root: &Path) -> PathBuf {
    root.join(".coderag").join("rust-index.json")
}

#[cfg(test)]
mod search_filter_tests {
    use super::*;
    use std::fs;

    fn write(dir: &std::path::Path, rel: &str, body: &str) {
        let path = dir.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, body).unwrap();
    }

    fn temp_repo(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "locus-{name}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn options(limit: usize) -> SearchOptions {
        SearchOptions::limit_only(limit)
    }

    #[test]
    fn refresh_mode_string_parses_as_auto() {
        assert_eq!(IndexMode::parse("refresh"), IndexMode::Auto);
        assert_eq!(IndexMode::parse("full"), IndexMode::Full);
        assert_eq!(IndexMode::parse("auto"), IndexMode::Auto);
    }

    #[test]
    fn final_extension_is_exact_and_case_sensitive() {
        assert_eq!(final_extension("src/a.ts"), Some("ts"));
        assert_eq!(final_extension("src/a.tsx"), Some("tsx"));
        assert_eq!(final_extension("src/a.js"), Some("js"));
        assert_eq!(final_extension("src/a.jsx"), Some("jsx"));
        assert_eq!(final_extension("src/a.h"), Some("h"));
        assert_eq!(final_extension("src/a.sh"), Some("sh"));
        assert_eq!(final_extension(r"src\a.sh"), Some("sh"));
        assert_eq!(final_extension("src/a.TS"), Some("TS"));
        assert_ne!(final_extension("src/a.sh"), Some("h"));
        assert_ne!(final_extension("src/a.tsx"), Some("ts"));
    }

    #[test]
    fn filters_run_before_limit_and_keep_full_index_document_frequency() {
        let dir = temp_repo("filters");
        write(&dir, "ext/a.ts", "export function extmarker() { return 1 }\n");
        write(&dir, "ext/a.tsx", "export function extmarker() { return 1 }\n");
        write(&dir, "ext/a.js", "export function extmarker() { return 1 }\n");
        write(&dir, "ext/a.jsx", "export function extmarker() { return 1 }\n");
        write(&dir, "ext/a.h", "int extmarker;\n");
        write(&dir, "ext/a.sh", "extmarker\n");
        write(&dir, "Src/Auth.ts", "export function markerpath() { return 1 }\n");
        write(
            &dir,
            "rank/first.ts",
            "export function zebra() { zebra zebra zebra zebra }\n",
        );
        write(&dir, "rank/second.ts", "export function zebra() { zebra }\n");
        write(&dir, "rank/third.ts", "export function zebra() { zebra }\n");
        write(&dir, "df/a.ts", "export function alpha() { sharedterm }\n");
        write(&dir, "df/b.ts", "export function beta() { sharedterm }\n");
        write(&dir, "df/c.ts", "export function gamma() { otherterm }\n");
        write(&dir, "paint.ts", "fn paint() { return 1 }\nexport function render() { return 2 }\n");
        write(&dir, "only-paint.ts", "fn paint() { return 1 }\n");
        write(
            &dir,
            "lib.rs",
            "// preambletoken\npub(crate) fn paint() {}\npub struct Color {}\npub enum Mode {}\npub trait Brush {}\n",
        );
        write(
            &dir,
            "main.go",
            "// gopreamble\nfunc main() {}\nfunc (s *Server) Listen() {}\n",
        );

        let (index, _) = build_index(&dir, 1_048_576).unwrap();
        let paths: Vec<_> = index.chunks.iter().map(|chunk| chunk.path.as_str()).collect();
        assert!(paths.iter().all(|path| !path.ends_with(".h") && !path.ends_with(".sh")));

        let ext_hits = |ext: &str| {
            search_index_with(
                &index,
                "extmarker",
                &SearchOptions {
                    file_extensions: vec![ext.into()],
                    ..options(10)
                },
            )
        };
        let only = |ext: &str, suffix: &str| {
            let hits = ext_hits(ext);
            assert_eq!(hits.len(), 1, "{ext}");
            assert!(hits[0].path.ends_with(suffix), "{} -> {}", ext, hits[0].path);
        };
        only("ts", "ext/a.ts");
        only(".ts", "ext/a.ts");
        only("tsx", "ext/a.tsx");
        only("js", "ext/a.js");
        only("jsx", "ext/a.jsx");
        assert!(ext_hits("TS").is_empty());
        assert!(ext_hits("h").is_empty());
        assert!(ext_hits("sh").is_empty());

        let auth = |filter: &str| {
            search_index_with(
                &index,
                "markerpath",
                &SearchOptions {
                    path_filter: Some(filter.into()),
                    ..options(10)
                },
            )
        };
        assert!(auth("auth").is_empty());
        assert_eq!(auth("Auth").len(), 1);
        assert_eq!(auth(r"Src\Auth").len(), 1);
        let excluded = search_index_with(
            &index,
            "markerpath",
            &SearchOptions {
                exclude_paths: vec!["Auth".into()],
                ..options(10)
            },
        );
        assert!(excluded.is_empty());

        let located = search_index_with(&index, "markerpath", &options(10));
        assert_eq!(located.len(), 1);
        assert!(located[0].snippet.as_ref().unwrap().contains("markerpath"));
        let hidden = search_index_with(
            &index,
            "markerpath",
            &SearchOptions {
                include_content: false,
                ..options(10)
            },
        );
        assert_eq!(hidden.len(), 1);
        assert!(hidden[0].snippet.is_none());

        let top = search_index_with(&index, "zebra", &options(1));
        assert_eq!(top.len(), 1);
        assert!(top[0].path.ends_with("rank/first.ts"), "{}", top[0].path);
        let next = search_index_with(
            &index,
            "zebra",
            &SearchOptions {
                limit: 1,
                exclude_paths: vec!["rank/first.ts".into()],
                ..options(1)
            },
        );
        assert_eq!(next.len(), 1);
        assert!(!next[0].path.contains("rank/first.ts"), "{}", next[0].path);

        let df_hits = search_index_with(
            &index,
            "sharedterm",
            &SearchOptions {
                path_filter: Some("df/a.ts".into()),
                ..options(10)
            },
        );
        assert_eq!(df_hits.len(), 1);
        let df = df_hits[0]
            .score_components
            .iter()
            .find(|part| part.term == "sharedterm")
            .unwrap();
        assert_eq!(df.document_frequency, 2.0);

        let paint_ts: Vec<_> = index
            .chunks
            .iter()
            .filter(|chunk| chunk.path == "paint.ts" || chunk.path == "only-paint.ts")
            .collect();
        assert!(paint_ts.iter().all(|chunk| chunk.symbol_name.as_deref() != Some("paint")));
        assert!(paint_ts.iter().any(|chunk| chunk.symbol_name.as_deref() == Some("render")));

        let rust_names: Vec<_> = index
            .chunks
            .iter()
            .filter(|chunk| chunk.path == "lib.rs")
            .filter_map(|chunk| chunk.symbol_name.clone())
            .collect();
        for name in ["paint", "Color", "Mode", "Brush"] {
            assert!(rust_names.iter().any(|found| found == name), "{rust_names:?}");
        }
        assert!(index
            .chunks
            .iter()
            .filter(|chunk| chunk.path == "lib.rs")
            .all(|chunk| chunk.text.contains("preambletoken")));

        let go_names: Vec<_> = index
            .chunks
            .iter()
            .filter(|chunk| chunk.path == "main.go")
            .filter_map(|chunk| chunk.symbol_name.clone())
            .collect();
        assert!(go_names.iter().any(|name| name == "main"), "{go_names:?}");
        assert!(go_names.iter().any(|name| name == "Listen"), "{go_names:?}");
        assert!(index
            .chunks
            .iter()
            .filter(|chunk| chunk.path == "main.go")
            .all(|chunk| chunk.text.contains("gopreamble")));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn extension_filter_does_not_treat_sh_as_h() {
        let chunks = vec![
            Chunk {
                path: "include/widget.h".into(),
                start_line: 1,
                end_line: 1,
                text: "headerterm".into(),
                tokens: tokenize("headerterm"),
                symbol_name: None,
                chunk_type: "file".into(),
            },
            Chunk {
                path: "scripts/widget.sh".into(),
                start_line: 1,
                end_line: 1,
                text: "headerterm".into(),
                tokens: tokenize("headerterm"),
                symbol_name: None,
                chunk_type: "file".into(),
            },
        ];
        let index = SearchIndex {
            root: "/tmp".into(),
            chunks,
            ..SearchIndex::default()
        };
        let hits = search_index_with(
            &index,
            "headerterm",
            &SearchOptions {
                file_extensions: vec!["h".into()],
                ..options(10)
            },
        );
        assert_eq!(hits.len(), 1);
        assert!(hits[0].path.ends_with(".h"));
        assert!(!hits[0].path.ends_with(".sh"));
    }
}
