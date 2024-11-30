use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "kono")]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    // // create a kono.app for .exe
    // Create,
    // // manage prefixes
    // Prefix {
    //     #[command(subcommand)]
    //     command: PrefixCommand,
    // },
    // // manage packages
    // Package {
    //     #[command(subcommand)]
    //     command: PackageCommand,
    // },
    // run an application
    Run { prog: String },
}

#[derive(Debug, Subcommand)]
pub enum PrefixCommand {
    // create a container
    Create,
    // migrate a container
    Migrate,
}

#[derive(Debug, Subcommand)]
pub enum PackageCommand {
    // install a package
    Install(Package),
    // uninstall a package
    Uninstall(Package),
}

#[derive(Debug, Args)]
pub struct Package {
    // package name
    #[arg(value_name = "PACKAGE")]
    pkg_name: String,
}
