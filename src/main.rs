mod cache;
mod commands;

use clap::Parser;

#[derive(clap::Parser)]
#[command(arg_required_else_help = true)]
/// Zig version mangager
pub struct Cli {
        #[command(subcommand)]
        pub command: Command,
}

#[derive(Debug, PartialEq, Eq, clap::Subcommand)]
pub enum Command {
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

        match cli.command {
                Command::Update => {
                        commands::update::execute();
                }

                Command::Check { version } => {
                        commands::check::execute(version);
                }
        }
}
