use std::process::{Command, Output};

fn run_elderheim(args: &[&str]) -> Option<Output> {
    Command::new(env!("CARGO_BIN_EXE_elderheim"))
        .args(args)
        .output()
        .ok()
}

#[test]
fn list_targets_prints_the_supported_one_zero_matrix() {
    let output = run_elderheim(&["--list-targets"]);
    assert!(output.is_some(), "elderheim binary should run");

    if let Some(output) = output {
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        let expected = [
            "linux-x86-elf32",
            "linux-x86_64-elf64",
            "linux-aarch32-elf32",
            "linux-aarch64-elf64",
            "windows-x86_64-pe64",
            "macos-aarch64-macho64",
        ];

        for target in expected {
            assert!(stdout.lines().any(|line| line == target));
        }
    }
}

#[test]
fn target_validation_accepts_supported_target() {
    let output = run_elderheim(&["--target", "linux-x86_64-elf64"]);
    assert!(output.is_some(), "elderheim binary should run");

    if let Some(output) = output {
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_eq!(stdout.trim(), "linux-x86_64-elf64");
    }
}

#[test]
fn target_validation_rejects_unsupported_combination() {
    let output = run_elderheim(&["--target", "windows-aarch64-pe64"]);
    assert!(output.is_some(), "elderheim binary should run");

    if let Some(output) = output {
        assert!(!output.status.success());

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("E-TARGET-UNSUPPORTED"));
        assert!(stderr.contains("target combination is not supported for 1.0"));
    }
}
