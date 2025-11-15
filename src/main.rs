mod arguments;
mod gui;
mod manifest;
mod paths;
mod runner;

use crate::gui::KonoGui;
use arguments::{Arguments, Command};
use clap::Parser;

fn main() {
    // load config and defaults if present
    let mut kono = KonoGui::initialize();

    // checks paths and create folders if necessary
    if let Err(err) = paths::ensure_kono_paths() {
        println!("Error creating folders!\n{}", err);
        std::process::exit(1);
    };

    // parse cli arguments
    let args = Arguments::parse();

    if let Some(command) = args.command {
        match command {
            Command::Run { prog: app } => runner::run(app, kono),
        }
        return;
    } else {
        // Launch full gui
        kono.open_gui();
    }
}
