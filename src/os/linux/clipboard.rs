use std::process::Command;

pub fn clipboard_commands(value: &str) -> Vec<Command> {
    let mut commands = vec![];

    let mut wl_copy = Command::new("wl-copy");
    wl_copy.arg("--").arg(value);
    commands.push(wl_copy);

    let mut xclip = Command::new("xclip");
    xclip
        .args(["-selection", "clipboard", "-n", "--"])
        .arg(value);
    commands.push(xclip);

    commands
}
