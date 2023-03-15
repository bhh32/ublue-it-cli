use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// Test the config values for each de and flag
#[test]
fn default_no_reboot() -> TestResult {
    run(&[])
}

#[test]
fn default_nvidia_no_reboot() -> TestResult {
    run(&["-n"])
}

#[test]
fn default_with_reboot() -> TestResult {
    run(&["-r"])
}

#[test]
fn default_nvidia_with_reboot() -> TestResult {
    run(&["-n", "-r"])
}

#[test]
fn explicit_gnome_no_reboot() -> TestResult {
    run(&["-d", "gnome"])
}

#[test]
fn explicit_gnome_nvidia_no_reboot() -> TestResult {
    run(&["-d", "gnome", "-n"])
}

#[test]
fn explicit_gnome_with_reboot() -> TestResult {
    run(&["-d", "gnome", "-n", "-r"])
}

#[test]
fn kinoite_no_reboot() -> TestResult {
    run(&["-d", "kde"])
}

#[test]
fn kinoite_with_reboot() -> TestResult {
    run(&["-d", "kde", "-r"])
}

#[test]
fn kinoite_nvidia_no_reboot() -> TestResult {
    run(&["-d", "kde", "-n"])
}

#[test]
fn kinoite_nvidia_with_reboot() -> TestResult {
    run(&["-d", "kde", "-n", "-r"])
}

#[test]
fn lxqt_no_reboot() -> TestResult {
    run(&["-d", "lxqt"])
}

#[test]
fn lxqt_with_reboot() -> TestResult {
    run(&["-d", "lxqt", "-r"])
}

#[test]
fn lxqt_nvidia_no_reboot() -> TestResult {
    run(&["-d", "lxqt", "-n"])
}

#[test]
fn lxqt_nvidia_with_reboot() -> TestResult {
    run(&["-d", "lxqt", "-r", "-n"])
}

#[test]
fn xfce_no_reboot() -> TestResult {
    run(&["-d", "xfce"])
}

#[test]
fn xfce_with_reboot() -> TestResult {
    run(&["-r", "-d", "xfce"])
}

#[test]
fn xfce_nvidia_no_reboot() -> TestResult {
    run(&["--has_nvidia", "--desktop_env", "xfce"])
}

#[test]
fn xfce_nvidia_with_reboot() -> TestResult {
    run(&["--auto-reboot", "-d", "xfce", "-n"])
}

#[test]
fn bluefin_no_reboot() -> TestResult {
    run(&["--desktop_env", "bluefin"])
}

#[test]
fn bluefin_with_reboot() -> TestResult {
    run(&["-r", "-d", "bluefin"])
}

#[test]
fn bluefin_nvidia_no_reboot() -> TestResult {
    run(&["-d", "bluefin", "-n"])
}

#[test]
fn bluefin_nvidia_with_reboot() -> TestResult {
    run(&["-d", "bluefin", "-r", "-n"])
}

// Helper Functions
fn run(args: &[&str]) -> TestResult {
    Command::cargo_bin("ublue-it_cli")?
        .args(args)
        .assert()
        .success();
    
    Ok(())
}