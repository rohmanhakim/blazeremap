//! CLI integration tests (without hardware)
use assert_cmd::cargo::cargo_bin_cmd;
use mockall::PredicateBooleanExt;

#[test]
fn test_cli_help() {
    let mut cmd = cargo_bin_cmd!("blazeremap");
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Linux keyboard-to-gamepad remapping software"))
        .stdout(predicates::str::contains("Usage:"))
        .stdout(predicates::str::contains("Commands:"))
        .stdout(predicates::str::contains("detect"))
        .stdout(predicates::str::contains("help"))
        .stdout(predicates::str::contains("Options:"))
        .stdout(predicates::str::contains("--help"))
        .stdout(predicates::str::contains("--version"));
}

#[test]
fn test_detect_help() {
    let mut cmd = cargo_bin_cmd!("blazeremap");
    cmd.arg("detect").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Detect gamepads"))
        .stdout(predicates::str::contains("connected to your computer"))
        .stdout(predicates::str::contains("--verbose"))
        .stdout(predicates::str::contains("-v"));
}

#[test]
fn test_no_subcommand_fails() {
    let mut cmd = cargo_bin_cmd!("blazeremap");

    cmd.assert().failure().stderr(predicates::str::contains("Usage:"));
}

#[test]
fn test_version_flag() {
    let mut cmd = cargo_bin_cmd!("blazeremap");
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("blazeremap"))
        .stdout(predicates::str::contains("0.1.0")); // Version from Cargo.toml
}

#[test]
fn test_invalid_subcommand() {
    let mut cmd = cargo_bin_cmd!("blazeremap");
    cmd.arg("invalid-command");

    cmd.assert().failure().stderr(
        predicates::str::contains("unrecognized subcommand")
            .or(predicates::str::contains("unexpected argument")),
    );
}

#[test]
fn test_detect_with_verbose_flag() {
    let mut cmd = cargo_bin_cmd!("blazeremap");
    cmd.arg("detect").arg("--verbose");

    // This will try to detect real hardware
    // We just check it doesn't crash
    cmd.assert()
        .success() // Should succeed even with no gamepads
        .stdout(predicates::str::contains("Detecting gamepads"));
}

#[test]
fn test_detect_short_verbose_flag() {
    let mut cmd = cargo_bin_cmd!("blazeremap");
    cmd.arg("detect").arg("-v");

    cmd.assert().success().stdout(predicates::str::contains("Detecting gamepads"));
}
