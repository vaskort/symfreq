use symfreq::{read_path, DEFAULT_EXTENSIONS};
use tempfile::tempdir;
use std::collections::HashSet;
use pretty_assertions::assert_eq;

#[test]
fn read_path_basic() {
    let exts: HashSet<&str> = DEFAULT_EXTENSIONS
        .iter()
        .copied()
        .collect();
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("test.rs"), "fn main() {}").unwrap();
    std::fs::write(dir.path().join("readme.txt"), "Hello").unwrap();
    let result = read_path(dir.path(), &exts).unwrap();

    assert!(result.content.contains("fn main() {}"));
    assert_eq!(result.files_read, 1);
    assert_eq!(result.files_failed, 0);
    assert_eq!(result.files_skipped, 1);
}
#[test]
fn read_path_no_extensions_found() {
    let exts = ["js"].iter().copied().collect();
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("test.rs"), "fn main() {}").unwrap();
    let result = read_path(dir.path(), &exts).unwrap();

    assert!(result.content.contains(""));
    assert_eq!(result.files_read, 0);
    assert_eq!(result.files_skipped, 1);
    assert_eq!(result.files_read, 0);
}
