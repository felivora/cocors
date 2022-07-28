use coco_git::core::{utility, Repository};
use std::env;
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    let repo = Repository::new(Path::new(path)).unwrap();

    println!(
        "{}",
        repo.log("main", "refactoring_krueger", "%h»¦«%cn»¦«%ce»¦«%ct»¦«%s»¦«%b")
            .unwrap()
    );
}
