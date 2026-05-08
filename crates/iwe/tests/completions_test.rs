use std::process::Command;

#[test]
fn test_completions_bash_outputs_content() {
    let output = Command::new(crate::common::get_iwe_binary_path())
        .args(["completions", "bash"])
        .output()
        .expect("Failed to execute iwe completions bash");

    assert!(output.status.success(), "completions bash should succeed");
    let stdout = String::from_utf8(output.stdout).expect("Valid UTF-8 stdout");
    assert!(!stdout.is_empty(), "Completion script should not be empty");
    assert!(
        stdout.contains("iwe"),
        "Completion script should reference the binary name"
    );
}

#[test]
fn test_completions_zsh_outputs_content() {
    let output = Command::new(crate::common::get_iwe_binary_path())
        .args(["completions", "zsh"])
        .output()
        .expect("Failed to execute iwe completions zsh");

    assert!(output.status.success(), "completions zsh should succeed");
    let stdout = String::from_utf8(output.stdout).expect("Valid UTF-8 stdout");
    assert!(!stdout.is_empty(), "Completion script should not be empty");
}

#[test]
fn test_completions_fish_outputs_content() {
    let output = Command::new(crate::common::get_iwe_binary_path())
        .args(["completions", "fish"])
        .output()
        .expect("Failed to execute iwe completions fish");

    assert!(output.status.success(), "completions fish should succeed");
    let stdout = String::from_utf8(output.stdout).expect("Valid UTF-8 stdout");
    assert!(!stdout.is_empty(), "Completion script should not be empty");
}

#[test]
fn test_completions_invalid_shell_fails() {
    let output = Command::new(crate::common::get_iwe_binary_path())
        .args(["completions", "nonexistent-shell"])
        .output()
        .expect("Failed to execute iwe completions");

    assert!(
        !output.status.success(),
        "completions with an unknown shell should fail"
    );
}
