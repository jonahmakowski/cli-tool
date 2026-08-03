use assert_cmd::Command;

#[test]
fn doctor_no_fail() {
    let output = Command::cargo_bin("cli-tool")
        .unwrap()
        .arg("doctor")
        .output()
        .unwrap();

    assert!(output.status.success());
}
