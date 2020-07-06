use std::process::Command;

pub fn execute(command: &String) -> bool {
    let mut result = Command::new("sh")
        .arg("-c")
        .arg(command.to_string())
        .spawn()
        .unwrap();

    result.wait().is_ok()
}
