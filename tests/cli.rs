use assert_cmd::Command;

#[test]
fn stdout() {
    let mut cmd = Command::cargo_bin("closefrom").unwrap();
    let output = cmd.args(["2", "sh", "-c", "echo test"]).assert();
    output.success().stdout("test\n");
}

#[test]
fn stderr() {
    let mut cmd = Command::cargo_bin("closefrom").unwrap();
    let output = cmd.args(["2", "sh", "-c", "echo test >&2"]).assert();
    output.failure();
}
