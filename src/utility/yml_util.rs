

use regex::Regex;

use super::sem_version::Version;

pub fn find_version(manifest: &str) -> Option<Version> {
    let version = Regex::new(r"version: (\d+\.\d+\.\d+(-.+)?)").unwrap();
    let caps = version.captures(manifest).unwrap();

    let version = caps.get(1).map_or(None, |m| Some(m.as_str().to_string()));

    if version.is_none() {
        eprintln!("No version could be found within the string");
        return None;
    }

    Version::parse(&version.unwrap())
}

#[cfg(test)]
mod tests {

    use crate::utility::{sem_version::Version, yml_util::find_version};

    #[test]
    fn parse_correct_semver_is_correct() {
        let semver_string = String::from("version: 10.9.756-demo");
        let semver_result = find_version(&semver_string);

        let semver = Version {
            major: 10,
            minor: 9,
            patch: 756,
            build: Some(String::from("demo").to_string()),
        };

        assert_eq!(semver_result.unwrap(), semver);
    }

    #[test]
    fn parse_correct_semver_is_parsed_some() {
        let semver_string = "10.9.756-demo";
        let semver_result = Version::parse(semver_string);

        assert!(semver_result.is_some());
    }

    #[test]
    fn parse_entire_yaml_with_version() {
        let yaml = "name: \"@ax/apax-build\"\nversion: 0.4.1\nauthor: Siemens AG\ndescription: Provides the functionality for the `apax build` command.\ntype: generic\nfiles:\n  - bin\n  - dist\n  - THIRD_PARTY_*\n_internalPackageJsonPassthrough:\n  bin: bin/cli.js\n  repository:\n    type: git\n    url: https://code.siemens.com/ax/apax/apax-build\ndependencies:\n  '@ax/third-party-licenses-apax-build': 0.4.1\n";
        let semver_result = find_version(yaml);

        let semver = Version {
            major: 0,
            minor: 4,
            patch: 1,
            build: None,
        };

        assert_eq!(semver_result.unwrap(), semver);
    }
}
