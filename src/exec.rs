use std::process::Command;

pub fn execute(command: &String) -> bool {
    let mut result = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .unwrap();

    result.wait().is_ok()
}
