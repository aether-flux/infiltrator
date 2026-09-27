use std::{env, process::Command};

/// Creates a command from the given string to be executed
pub fn create_command(cmd_str: &str) -> Command {
    let shell = env::var("SHELL").unwrap_or("/bin/sh".to_string());

    let mut cmd = Command::new(shell);
    cmd.arg("-c").arg(cmd_str);

    cmd
}
