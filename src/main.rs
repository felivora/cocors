mod cli;

use clap::Parser;

use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

/// Simple program to automatically apply semantic versioning based on the conventional commits specification

fn main() {
    let args = cli::Args::parse();

    let mut log_filter = LevelFilter::Info;

    if args.verbose {
        log_filter = LevelFilter::Debug;
    }
    if args.debug {
        log_filter = LevelFilter::Trace;
    }

    TermLogger::init(
        log_filter,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    match args.command {
        cli::commands::Commands::Lint(args) => {
            args.lint();
        }
    }
}
