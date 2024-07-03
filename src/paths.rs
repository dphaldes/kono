use std::{env, process::exit};

pub const HOME: &str = "HOME";
pub const XDG_DATA_PATH: &str = "XDG_DATA_PATH";

pub fn ensure_kono_paths() -> std::io::Result<()> {
    std::fs::create_dir_all(data_dir())?;
    std::fs::create_dir_all(package_dir())?;
    std::fs::create_dir_all(runner_dir())?;

    Ok(())
}

pub fn data_dir() -> String {
    let dir = match env::var(XDG_DATA_PATH) {
        Ok(val) => val,
        Err(_) => match env::var(HOME) {
            Ok(path) => format!("{}/.local/share", path),
            Err(_) => {
                println!("Error: Cannot get user $HOME");
                exit(1);
            }
        },
    };
    return format!("{}/kono", dir);
}

pub fn package_dir() -> String {
    return format!("{}/packages", data_dir());
}

pub fn runner_dir() -> String {
    return format!("{}/runner", data_dir());
}
