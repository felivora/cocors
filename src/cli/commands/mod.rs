use clap::Subcommand;

mod lint;

pub use lint::Lint;
#[derive(Subcommand)]
pub enum Commands {
    Lint(Lint),
}
