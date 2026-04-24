use std::io;
use std::process::{Child, Command, Stdio};

/// Get the pager command from $PAGER env var, or fall back to a
/// platform-appropriate default ("less -R" on Unix, "more" on Windows,
/// which ships with the OS).
fn get_pager_command() -> (String, Vec<String>) {
    if let Ok(pager) = std::env::var("PAGER") {
        let parts: Vec<&str> = pager.split_whitespace().collect();
        if let Some((cmd, args)) = parts.split_first() {
            return (
                cmd.to_string(),
                args.iter().map(|s| s.to_string()).collect(),
            );
        }
    }
    #[cfg(windows)]
    {
        ("more".to_string(), Vec::new())
    }
    #[cfg(not(windows))]
    {
        ("less".to_string(), vec!["-R".to_string()])
    }
}

/// Spawn a pager process
pub fn spawn_pager() -> io::Result<Child> {
    let (cmd, args) = get_pager_command();
    Command::new(&cmd).args(&args).stdin(Stdio::piped()).spawn()
}
