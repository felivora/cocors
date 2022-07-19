mod utility;
mod adapter;
use std::path::PathBuf;

use clap::Parser;

use utility::{
    fs_helper::read_manifest, sem_version::Version,
    yml_util::find_version,
};
use adapter::{apax_adapter::set_apax_version};

use crate::utility::commit::ConventionalCommit;

/// Simple program to automatically apply semantic versioning based on the conventional commits specification
#[derive(Parser, Debug, Clone)]
#[clap(author, about)]
struct Args {
    /// Path of the apax project that contains an apax.yml where the version will be bumped
    #[clap(short, long)]
    version: Option<String>,

    /// Path of the apax project that contains an apax.yml where the version will be bumped
    #[clap(parse(from_os_str), short, long)]
    path: Option<std::path::PathBuf>,

    /// Commit message, following the conventional commit spec where the keywords will be used to determine the new version
    #[clap(short, long)]
    commit_message: String,

    /// Uses the commit message to rollback to the previous version instead of bumping the version ahead
    #[clap(parse(from_flag), long)]
    rollback: bool,

    /// Update the manifest file with the new version, skipping this flag will only return the new value
    #[clap(parse(from_flag), long)]
    update: bool,
}

fn main() {
    let args = Args::parse();

    if args.path.is_none() && args.version.is_none() {
        eprintln!("Either the version or a path must be given.");
        std::process::exit(exitcode::USAGE);
    }

    if args.path.is_some() {
        let version = from_file(args.clone());
        print!("{}", version);
        std::process::exit(exitcode::OK);
    }

    if args.version.is_some() {
        let version_opt = Version::parse(args.version.unwrap().as_str());

        if version_opt.is_none() {
            eprintln!("The version could not be properly parsed into a semantic version, check if is correct");
            std::process::exit(exitcode::USAGE);
        }

        let commit = ConventionalCommit::new(args.commit_message.as_str());

        if commit.is_none() {
            eprintln!("Commit message could not be parsed, make sure that it follows the conventional commit specification");
            std::process::exit(exitcode::USAGE);
        }

        let mut version = version_opt.unwrap();

        if args.rollback {
            version.rollback(&commit.unwrap());
        } else {
            version.bump(&commit.unwrap());
        }

        print!("{}", version);
        std::process::exit(exitcode::OK);
    }
}

fn from_file(args: Args) -> Version {
    let manifest: String;
    let path: PathBuf;
    match read_manifest(args.path.unwrap()) {
        Some(x) => {
            manifest = x.0;
            path = x.1
        }
        None => {
            eprintln!("No apax.yml file could be found in the given directory or any subdirectories. Check if path is a valid apax package.");
            std::process::exit(exitcode::OSFILE);
        }
    }

    let version_opt = find_version(manifest.as_str());

    if version_opt.is_none() {
        eprintln!("The version found in the apax.yml could not be properly parsed into a semantic version, check if is correct");
        std::process::exit(exitcode::DATAERR);
    }

    let mut version = version_opt.unwrap();

    let commit = ConventionalCommit::new(args.commit_message.as_str());

    if commit.is_none() {
        eprintln!("Commit message could not be parsed, make sure that it follows the conventional commit specification");
        std::process::exit(exitcode::DATAERR);
    }

    if args.rollback {
        version.rollback(&commit.unwrap())
    } else {
        version.bump(&commit.unwrap())
    }

    if args.update {
        // manifest = update_version(manifest, &version).unwrap();

        // update_manifest(manifest, path);
        set_apax_version(&version, path).expect("Could not successfully update the apax.yml");
    }

    version
}

#[cfg(test)]
mod test;
