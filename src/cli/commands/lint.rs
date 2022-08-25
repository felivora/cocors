use std::path::PathBuf;

use clap::{ArgGroup, Args, Parser, ValueEnum};
use coco::{lint::Level, Commit};
use coco_git::core::Repository;
use log::{debug, error, info, trace, warn};

#[derive(Args)]
#[clap(group(
    ArgGroup::new("source").required(true).args(&["commit-message", "path"])
))]
/// Lints a conventional commit message or the message of the last git commit
pub struct Lint {
    /// Defines the level at which the linter will return an error
    #[clap(arg_enum, value_parser)]
    pub level: Option<Level>,

    /// The specific conventional commit message you want to lint
    #[clap(short, long = "commit-message")]
    pub commit_message: Option<String>,

    /// The path to the repository where the last message will be linted
    #[clap(short, long, value_parser)]
    pub path: Option<PathBuf>,
}

impl Lint {
    pub fn lint(&self) {
        trace!("Starting linting functionality");

        let mut commit_to_lint = String::new();

        if self.commit_message.is_some() {
            commit_to_lint = self.commit_message.clone().unwrap();
            trace!("Linting the provided specific conventional commit message");
            trace!("Message is: \t{}", commit_to_lint);
        }
        trace!("No specific conventional commit message provided");

        if self.path.is_some() {
            let path = self.path.clone().unwrap();
            match Repository::new(path.as_path()) {
                Ok(r) => {
                    trace!("Provided repository, working in root");
                }
                Err(_) => {
                    error!(
                        "Given path \"{}\" is not a repository",
                        path.to_string_lossy()
                    );
                }
            }
        }

        let lint_result = Commit::lint(&commit_to_lint);

        if lint_result.lints.is_empty() {
            info!("✔️ Your commit is flawless, go ahead an push!");
        } else {
            for lint in lint_result.lints {
                match lint.level {
                    Level::Error => error!("{}", lint),
                    Level::Warning => warn!("{}", lint),
                    _ => info!("{}", lint),
                };
            }
        }

        if lint_result.commit.is_none() {
            std::process::exit(exitcode::DATAERR);
        }
    }
}
