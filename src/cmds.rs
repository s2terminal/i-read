use regex::Regex;
use std::fmt;

pub struct Command {
    raw_string: String
}

impl Command {
    fn pattern() -> Regex {
        Regex::new(r"^\s*[#>$]+\s*").unwrap()
    }

    pub fn executable(&self) -> String {
        Self::pattern().replace(&self.raw_string, "").into_owned()
    }

    pub fn get_commands(content: String) -> Vec<Command> {
        let mut commands: Vec<Command> = Vec::new();
        for line in content.lines() {
            if Self::pattern().is_match(line) {
                commands.push(Command { raw_string: String::from(line) })
            }
        }
        commands
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.raw_string)
    }
}
