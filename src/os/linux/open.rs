use std::{
    ffi::{OsStr, OsString},
    process::Command,
};

fn option_safe_path(path: &OsStr) -> OsString {
    use std::os::unix::ffi::OsStrExt;

    if path.as_bytes().first() == Some(&b'-') {
        let mut safe = OsString::from("./");
        safe.push(path);
        safe
    } else {
        path.to_os_string()
    }
}

pub fn commands<T: AsRef<OsStr>>(path: T) -> Vec<Command> {
    let path = path.as_ref();
    let mut commands = vec![];

    let safe_path = option_safe_path(path);

    let mut xdg = Command::new("xdg-open");
    xdg.arg(&safe_path);
    commands.push(xdg);

    let mut gio = Command::new("gio");
    gio.arg("open").arg(&safe_path);
    commands.push(gio);

    let mut gnome = Command::new("gnome-open");
    gnome.arg(&safe_path);
    commands.push(gnome);

    let mut kde = Command::new("kde-open");
    kde.arg("--").arg(path);
    commands.push(kde);

    commands
}
