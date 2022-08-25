use clap::Parser;

use super::commands::Commands;

#[derive(Parser)]
/// Convenience & pipeline functionality for conventional commits
///
/// Provides functionality for linting, collecting and processing conventional commit messages.
/// Can be used in a repository to pull commit messages in a range for changelog generation.
pub struct Args {
    /// Turn on debug information, this will result in a very verbose output, which might
    /// not be desired in productive mode and only be useful when there is an issue
    #[clap(short, long)]
    pub debug: bool,

    /// Activate a more verbose output to trace what cocors is doing
    #[clap(short, long)]
    pub verbose: bool,

    #[clap(subcommand)]
    pub command: Commands,
}
