import type { SearchResult } from '@sylphx/coderag'

type SearchResultWithLocators = SearchResult & {
	matchedLines?: number[]
	symbolName?: string
}

export type RetrievalRoute = 'tfidf' | 'semantic' | 'rust-tfidf' | 'rust-semantic-hybrid'

export interface RetrievalEngineEvidence {
	contract_version: string
	index: 'rust-tfidf' | 'typescript'
	search: RetrievalRoute
}

export interface RetrievalLocator {
	path: string
	startLine?: number
	endLine?: number
	matchedLines?: number[]
}

export interface ScoreComponentEvidence {
	term: string
	termFrequency: number
	documentFrequency: number
	idf: number
	bm25: number
}

export interface RetrievalResultEvidence {
	path: string
	locator: RetrievalLocator
	score: number
	matchedTerms: string[]
	scoreComponents?: ScoreComponentEvidence[]
	route: RetrievalRoute
	confidence: 'deterministic' | 'derived' | 'inferred' | 'unknown'
	snippet?: string
	symbolName?: string
	chunkType?: string
	language?: string
}

export interface CodebaseSearchEnvelope {
	status: 'ok' | 'error'
	subject: 'codebase_search'
	query: string
	route: RetrievalRoute
	engine: RetrievalEngineEvidence
	freshness: {
		indexedFiles: number
		indexing: boolean
		stale: boolean
	}
	results: RetrievalResultEvidence[]
	warnings: string[]
	gaps: string[]
	nextActions: string[]
}

export function buildCodebaseSearchEnvelope(input: {
	query: string
	route: RetrievalRoute
	engine: RetrievalEngineEvidence
	indexedFiles: number
	indexing: boolean
	results: SearchResult[]
	warnings?: string[]
	gaps?: string[]
}): CodebaseSearchEnvelope {
	return {
		status: 'ok',
		subject: 'codebase_search',
		query: input.query,
		route: input.route,
		engine: input.engine,
		freshness: {
			indexedFiles: input.indexedFiles,
			indexing: input.indexing,
			stale: false,
		},
		results: input.results.map((result) => {
			const located = result as SearchResultWithLocators
			return {
				path: result.path,
				locator: {
					path: result.path,
					startLine: result.startLine,
					endLine: result.endLine,
					matchedLines: located.matchedLines,
				},
				score: result.score,
				matchedTerms: result.matchedTerms ?? [],
				...(result.scoreComponents && result.scoreComponents.length > 0
					? { scoreComponents: result.scoreComponents }
					: {}),
				route: input.route,
				confidence: 'deterministic' as const,
				snippet: result.snippet,
				symbolName: located.symbolName,
				chunkType: result.chunkType,
				language: result.language,
			}
		}),
		warnings: input.warnings ?? [],
		gaps: input.gaps ?? [],
		nextActions: [
			'Open the top result path and verify the cited line range before editing.',
			'Re-run codebase_search after index refresh if files changed.',
		],
	}
}

export function mapRustHitsToSearchResults(
	hits: Array<{
		path: string
		score: number
		matchedTerms: string[]
		scoreComponents?: Array<{
			term: string
			termFrequency: number
			documentFrequency: number
			idf: number
			bm25: number
		}>
		startLine?: number
		endLine?: number
		matchedLines?: number[]
		snippet?: string
		symbolName?: string
		chunkType?: string
	}>
): SearchResult[] {
	return hits.map((hit) => ({
		path: hit.path,
		score: hit.score,
		matchedTerms: hit.matchedTerms,
		...(hit.scoreComponents && hit.scoreComponents.length > 0
			? { scoreComponents: hit.scoreComponents }
			: {}),
		size: hit.snippet?.length ?? 0,
		startLine: hit.startLine,
		endLine: hit.endLine,
		matchedLines: hit.matchedLines,
		snippet: hit.snippet,
		...(hit.symbolName ? { symbolName: hit.symbolName } : {}),
		...(hit.chunkType || hit.symbolName ? { chunkType: hit.chunkType ?? 'function' } : {}),
	}))
}
