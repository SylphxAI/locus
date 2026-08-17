use coderag_mcp_server::{cli_bridge, http_transport, CoderagMcp, SERVER_VERSION};
use rmcp::ServiceExt;

fn root_from_args(args: &[String]) -> anyhow::Result<Option<String>> {
    let mut root = None;
    let mut index = 0;
    while index < args.len() {
        let argument = &args[index];
        if let Some(value) = argument.strip_prefix("--root=") {
            if value.trim().is_empty() {
                anyhow::bail!("--root requires a non-empty directory path");
            }
            root = Some(value.to_string());
        } else if argument == "--root" {
            index += 1;
            let value = args
                .get(index)
                .filter(|value| !value.trim().is_empty())
                .ok_or_else(|| anyhow::anyhow!("--root requires a directory path"))?;
            root = Some(value.clone());
        }
        index += 1;
    }
    Ok(root)
}

fn configure_from_args(args: &[String]) -> anyhow::Result<()> {
    if let Some(root) = root_from_args(args)? {
        std::env::set_var("CODERAG_ROOT", root);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    configure_from_args(&args)?;

    if args.iter().any(|argument| argument == "doctor") {
        eprintln!(
            "Locus Rust MCP server {SERVER_VERSION} ({})",
            coderag_core::ENGINE_NAME
        );
        if let Some(cli) = cli_bridge::resolve_cli_binary() {
            eprintln!("engine cli: {}", cli.display());
        } else {
            eprintln!("engine cli: unavailable (run `bun run build:rust`)");
        }
        return Ok(());
    }

    if http_transport::transport_from_env().is_some() {
        return http_transport::serve_http(http_transport::HttpConfig::from_env()).await;
    }

    let service = CoderagMcp::new().serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::root_from_args;

    #[test]
    fn accepts_equals_and_split_root_arguments() {
        assert_eq!(
            root_from_args(&["--root=/tmp/project".into()]).expect("root"),
            Some("/tmp/project".into())
        );
        assert_eq!(
            root_from_args(&["--root".into(), "/tmp/project".into()]).expect("root"),
            Some("/tmp/project".into())
        );
    }

    #[test]
    fn rejects_missing_root_value() {
        assert!(root_from_args(&["--root".into()]).is_err());
        assert!(root_from_args(&["--root=".into()]).is_err());
    }
}
