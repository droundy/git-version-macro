git_version_macro::declare!(my_version);

#[test]
fn is_right() {
    let vec = std::process::Command::new("git")
            .args(&["describe", "--always", "--dirty"])
            .output()
        .expect("failed to execute git").stdout;
    let name = std::str::from_utf8(&vec[..vec.len()-1]).expect("non-utf8 error?!");
    assert_eq!(my_version, name);
}
