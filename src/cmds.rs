use pulldown_cmark::{Parser, Tag, Options, Event};
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
        let options = Options::empty();
        let parser = Parser::new_ext(content.as_str(), options);
        let mut is_code: bool = false;
        for event in parser {
            match event {
                Event::Start(Tag::CodeBlock(_)) => is_code = true,
                Event::End(Tag::CodeBlock(_))   => is_code = false,
                Event::Text(text) => ( if is_code {
                    for line in text.lines() {
                        commands.push(Command { raw_string: String::from(line) });
                    }
                } ),
                _ => ()
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
