mod cache;
mod cmd;

use clap::Parser;

#[derive(clap::Parser)]
/// Zig version mangager
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Debug, PartialEq, Eq, clap::Subcommand)]
pub enum Cmd {
    /// Update local cache
    Update,

    /// Check available versions or a specific version
    Check {
        /// Specific version
        version: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Update => {
            cmd::update::execute();
        }

        Cmd::Check { version } => {
            cmd::check::execute(version);
        }
    }
}
