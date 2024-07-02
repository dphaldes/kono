mod arguments;
mod paths;
mod runner;

use clap::Parser;

use arguments::{Arguments, Command};

fn main() {
    // initialize

    // load config and defaults if present

    // checks paths and create folders if necessary
    if let Err(err) = paths::ensure_kono_paths() {
        println!("Error creating folders!\n{}", err);
        std::process::exit(1);
    };

    // cli
    let args = Arguments::parse();
    println!("{:?}", args);

    // act on arguments
    match args.command {
        Command::Run { prog: app } => runner::run(app),
        _ => todo!(),
    };
}
