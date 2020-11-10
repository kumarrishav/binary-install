use assert_cmd::Command;

#[test]
fn it_can_echo_stdin() {
    let message = "i love to write code it is so fun wow";
    get_bin()
        .arg("echo")
        .write_stdin(message)
        .assert()
        .stdout(predicates::str::contains(message))
        .success();
}

fn get_bin() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}
