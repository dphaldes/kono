mod manifest;

use std::path::{Path, PathBuf};

use dialog::DialogBox;

use crate::{paths, runner::manifest::Manifest};

const EXTENSIONS: [&str; 2] = ["exe", "msi"];

pub fn run(exe: String) {
    // check if the program exists and is valid
    let path = Path::new(&exe);
    if !path.exists() || !valid_exe(&path) {
        println!("Unsupported exectuble");
        return;
    }

    let exe_path = std::fs::canonicalize(path).unwrap();
    let exe_name = exe_path.file_stem().unwrap().to_str().unwrap();
    let exe_dir = exe_path.parent().unwrap();

    // check for config
    let manifest_path = exe_dir.join(format!("{}.kono.toml", exe_name));
    if !manifest_path.exists() {
        println!("Config not present");
        dialog::Message::new("No manifest found.")
            .title("Kono-Launch Error")
            .show_with(dialog::backends::KDialog::new())
            .expect("Dialog Error");
        return;
    }

    if let Ok(contents) = std::fs::read_to_string(manifest_path) {
        //load manifest
        let manifest: Manifest = match toml::from_str(&contents) {
            Ok(it) => it,
            Err(e) => {
                println!("Error: Unsupported manifest type");
                println!("Error: {}", e);
                return;
            }
        };
        println!("Manifest loaded : {:?}", manifest);

        let manifest = match manifest {
            Manifest::Kono(it) => it,
        };

        //check if prefix is valid
        let prefix_path = Path::new(&manifest.prefix);
        if !prefix_path.exists() {
            println!("Prefix doesn't exist");
            return;
        }

        //check if runner is valid
        let runner_dir = paths::runner_dir();
        let runner_path = Path::new(&runner_dir).join(manifest.runner);
        if !runner_path.exists() {
            println!("Runner doesn't exist");
            return;
        }

        // launch program
        println!(
            "Running {:?} inside prefix {:?} and runner {:?}",
            exe_path, prefix_path, runner_path
        );

        launch_exe_with_wine(exe_path, prefix_path.to_path_buf(), runner_path);
    };
}

fn valid_exe(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        if let Some(ext) = ext.to_str() {
            return EXTENSIONS.contains(&ext);
        }
    };
    false
}

fn launch_exe_with_wine(exe_path: PathBuf, prefix_path: PathBuf, runner_path: PathBuf) {
    std::process::Command::new(runner_path.join("bin/wine"))
        .arg(exe_path)
        .env("WINEPREFIX", prefix_path)
        .spawn()
        .expect("Error: Failed to start executable");
}
