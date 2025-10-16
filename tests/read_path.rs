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

#[cfg(unix)]
#[test]
fn read_path_file_read_fails() {
    use std::os::unix::fs::PermissionsExt;

    let exts: HashSet<&str> = ["rs"].iter().copied().collect();
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("secret.rs");

    // Create file
    std::fs::write(&file_path, "secret code").unwrap();

    // Make it unreadable (permissions = 000)
    let perms = std::fs::Permissions::from_mode(0o000);
    std::fs::set_permissions(&file_path, perms).unwrap();

    let result = read_path(dir.path(), &exts).unwrap();

    // What should files_failed be?
    assert_eq!(result.files_failed, 1);
}

#[test]
fn read_path_file_without_extension() {
    let exts = ["js"].iter().copied().collect();
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("Makefile"), "mock makefile contents").unwrap();
    let result = read_path(dir.path(), &exts).unwrap();

    assert_eq!(result.files_skipped, 1);
}
#[test]
fn read_path_nonexistent_directory() {
    let exts: HashSet<&str> = ["rs"].iter().copied().collect();
    let result = read_path("/path/that/does/not/exist/at/all", &exts).unwrap();

    assert_eq!(result.files_read, 0);
    assert_eq!(result.files_skipped, 0);
    assert_eq!(result.files_failed, 0);
}
