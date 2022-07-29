use coco_git::core::{utility, Repository};
use std::env;
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    let repo = Repository::new(Path::new(path)).unwrap();

    for tag in repo.tags().unwrap() {
        println!("{}", tag);
    }

    for commit in repo
        .log(
            "HEAD",
            repo.latest_tag().unwrap().as_str(),
            "%h»¦«%cn»¦«%ce»¦«%ct»¦«%s»¦«%b",
        )
        .unwrap()
        .lines()
    {
        println!("{}", commit);
    }
}
