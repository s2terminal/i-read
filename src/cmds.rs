use regex::Regex;

pub fn get_commands(content: String) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();
    let cmd_pattern = Regex::new(r"^\s*[#>$]+\s*").unwrap();
    for line in content.lines() {
        if cmd_pattern.is_match(line) {
            commands.push(cmd_pattern.replace(line, "").into_owned());
        }
    }

    commands
}
