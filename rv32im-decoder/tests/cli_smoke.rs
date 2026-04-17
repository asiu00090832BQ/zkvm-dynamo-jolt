use std::process::Command;

#[test]
fn decodes_hex_word_from_cli() {
    let output = Command::new(env!("CARGO_BIN_EXE_rv32im-decoder"))
        .arg("0x00c585b3")
        .output()
        .expect("binary should run");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0x00c585b3 =>"));
}
