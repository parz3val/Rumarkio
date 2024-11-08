use std::process::{Command, Stdio};

fn main() {
    let output = Command::new("echo")
        .arg("Hello, world")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute teh command");
    let stdout = String::from_utf8(output.stdout).expect("Output was not utf-8");
    println!("String==> {}", stdout);
}
