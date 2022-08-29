use std::path::PathBuf;

use clap::{ArgGroup, Args};
use coco::{
    lint::{Level, LintResult},
    Commit,
};
use coco_git::core::Repository;
use log::{error, info, trace, warn};

#[derive(Args)]
#[clap(group(
    ArgGroup::new("source").required(true).args(&["message", "path"])
))]
/// Lints a conventional commit message or the message of the last git commit
pub struct Lint {
    /// Defines the level at which the linter will return an error
    #[clap(arg_enum, value_parser)]
    pub level: Option<Level>,

    /// The specific conventional commit message you want to lint
    #[clap(short, long = "message")]
    pub message: Option<String>,

    /// The path to the repository where the last message will be linted
    #[clap(short, long, value_parser)]
    pub path: Option<PathBuf>,

    /// Flag on how many commit messages of the repository shall be linted
    #[clap(short, value_parser, requires = "path")]
    pub count: Option<usize>,

    /// Flag that defines how failures in multiple commit messages should be handeled
    /// If true, the tool will be kind and return a zero code on failure if there are multiple messages
    #[clap(short, long = "ignore-errors")]
    pub ignore_errors: bool,

    // Flag that, if set to true, will filter commit messages that are without errors
    #[clap(short, long = "only-error")]
    pub only_error: bool,
}

impl Lint {
    pub fn lint(&self) {
        trace!("Starting linting functionality");

        let mut commit_to_lint = String::new();

        if self.message.is_some() {
            commit_to_lint = self.message.clone().unwrap();
            trace!("Linting the provided specific conventional commit message");
            trace!("Message is: \t{}", commit_to_lint);
        }
        trace!("No specific conventional commit message provided");

        if self.path.is_some() {
            let path = self.path.clone().unwrap();
            match Repository::new(path.as_path()) {
                Ok(r) => {
                    trace!(
                        "Provided repository, working in root {}",
                        Repository::repo_root(path.as_path()).unwrap_or(String::from("undefined"))
                    );
                    let commit_res =
                        r.log("HEAD", "", "%h»¦«%cn»¦«%ce»¦«%ct»¦«%s»¦«%»»»", self.count);

                    if commit_res.is_err() {
                        error!("Provided repository does not contain any commits!");
                        std::process::exit(exitcode::DATAERR);
                    }

                    commit_to_lint = commit_res.unwrap();

                    trace!("Using commit messages for lint: {}", commit_to_lint);
                }
                Err(_) => {
                    error!(
                        "Given path \"{}\" is not a repository",
                        path.to_string_lossy()
                    );
                    std::process::exit(exitcode::DATAERR);
                }
            }
        }

        let mut success = true;
        let mut count = usize::default();

        for commit in commit_to_lint.as_str().split("»»»") {
            trace!("Linting message {}", commit);
            count += 1;

            if commit.trim().is_empty() {
                continue;
            };

            let mut details = commit.split("»¦«");

            let hash = details.nth(0);
            let message = details.nth(3);

            let lint_result = Commit::lint(&commit);
            success &= print_lint_result(lint_result, hash, message, self.only_error);
        }

        if !success {
            if self.ignore_errors && count > 1 {
                return;
            }

            std::process::exit(exitcode::DATAERR);
        }
    }
}

// returns false if the lint encountered a critical error
fn print_lint_result(
    lint_result: LintResult,
    hash: Option<&str>,
    message: Option<&str>,
    only_error: bool,
) -> bool {
    if lint_result.lints.is_empty() && lint_result.commit.is_some() {
        if !only_error {
            println!(
                "--------------------------------------------------------------------------\n"
            );
            info!(
                "✔️ : Your commit \"{}\" \"{}\" \n\t\t   is flawless, go ahead an push! ",
                hash.map_or_else(String::new, |h| h.trim().to_string()),
                message.map_or_else(String::new, |m| format!("{}", m.trim()))
            );
        }
    } else {
        println!("--------------------------------------------------------------------------\n");
        info!(
            "🤓  Some remarks on commit \"{}\" \"{}\" \n ",
            hash.map_or_else(String::new, |h| h.trim().to_string()),
            message.map_or_else(String::new, |m| format!("{}", m.trim()))
        );
        for lint in lint_result.lints {
            match lint.level {
                Level::Error => error!("{}", lint),
                Level::Warning => {
                    warn!("{}", lint)
                }
                _ => {
                    info!("{}", lint)
                }
            };
        }
    }

    println!("");

    if lint_result.commit.is_none() {
        return false;
    }

    return true;
}
