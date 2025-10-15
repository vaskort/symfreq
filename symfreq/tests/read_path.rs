use symfreq::{read_path, DEFAULT_EXTENSIONS};
use tempfile::tempdir;
use std::collections::HashSet;

#[test]
fn read_path_basic() {
    let exts: HashSet<&str> = DEFAULT_EXTENSIONS
        .iter()
        .copied()
        .collect();
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("test.rs"), "fn main() {}").unwrap();
    std::fs::write(dir.path().join("readme.txt"), "Hello").unwrap();
    let result = read_path(dir.path(), &exts);

    assert!(result.unwrap().contains("fn main() {}"))
}
