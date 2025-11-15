use std::{env, process::exit};

const HOME: &str = "HOME";
const XDG_DATA_PATH: &str = "XDG_DATA_PATH";

pub fn ensure_kono_paths() -> std::io::Result<()> {
    std::fs::create_dir_all(data_dir())?;
    std::fs::create_dir_all(components_dir())?;

    Ok(())
}

pub fn data_dir() -> String {
    let dir = match env::var(XDG_DATA_PATH) {
        Ok(val) => val,
        Err(_) => match env::var(HOME) {
            Ok(path) => format!("{}/.local/share", path),
            Err(_) => {
                eprintln!("Cannot get user $HOME");
                exit(1);
            }
        },
    };
    return format!("{}/kono", dir);
}

// pub fn prefix_dir() -> String {
//     return format!("{}/prefix", data_dir());
// }

pub fn components_dir() -> String {
    return format!("{}/components", data_dir());
}
