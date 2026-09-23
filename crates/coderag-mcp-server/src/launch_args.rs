use std::sync::OnceLock;

/// Arguments accepted by the `npx @sylphx/locus` process before it serves.
///
/// This parser is pure: it does not read or write process environment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchArgs {
    pub doctor: bool,
    pub root: Option<String>,
}

pub fn parse_launch_args<I, S>(args: I) -> Result<LaunchArgs, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut doctor = false;
    let mut root = None;
    let mut args = args.into_iter();
    while let Some(raw) = args.next() {
        let arg = raw.as_ref();
        if arg == "doctor" {
            doctor = true;
            continue;
        }
        if let Some(value) = arg.strip_prefix("--root=") {
            if value.is_empty() {
                return Err("--root requires a path".to_string());
            }
            root = Some(value.to_string());
            continue;
        }
        if arg == "--root" {
            let Some(next) = args.next() else {
                return Err("--root requires a path".to_string());
            };
            let value = next.as_ref();
            if value.is_empty() || value.starts_with('-') {
                return Err("--root requires a path".to_string());
            }
            root = Some(value.to_string());
            continue;
        }
        return Err(format!("unknown argument: {arg}"));
    }
    Ok(LaunchArgs { doctor, root })
}

static LAUNCH_ROOT: OnceLock<String> = OnceLock::new();

/// Remember the `--root` path for this process. Does not touch the environment.
pub fn set_launch_root(root: String) -> Result<(), String> {
    LAUNCH_ROOT
        .set(root)
        .map_err(|_| "launch root is already set".to_string())
}

pub fn launch_root() -> Option<String> {
    LAUNCH_ROOT.get().cloned()
}

#[cfg(test)]
mod tests {
    use super::parse_launch_args;

    #[test]
    fn parses_root_equals_and_split_forms() {
        let equals = parse_launch_args(["--root=/tmp/repo"]).unwrap();
        assert_eq!(equals.root.as_deref(), Some("/tmp/repo"));
        assert!(!equals.doctor);

        let split = parse_launch_args(["--root", "/tmp/repo", "doctor"]).unwrap();
        assert_eq!(split.root.as_deref(), Some("/tmp/repo"));
        assert!(split.doctor);
    }

    #[test]
    fn no_arguments_serve_without_doctor_or_root() {
        let parsed = parse_launch_args(std::iter::empty::<&str>()).unwrap();
        assert!(!parsed.doctor);
        assert_eq!(parsed.root, None);
    }

    #[test]
    fn rejects_a_missing_root_path() {
        assert!(parse_launch_args(["--root"]).is_err());
        assert!(parse_launch_args(["--root="]).is_err());
        assert!(parse_launch_args(["--root", "--other"]).is_err());
        assert!(parse_launch_args(["--max-size=1048576"]).is_err());
    }
}
