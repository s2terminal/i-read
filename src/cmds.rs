use pulldown_cmark::{Event, Options, Parser, Tag};
use regex::Regex;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Command {
    raw_string: String,
    #[allow(dead_code)]
    executable: bool,
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
        let mut heading_level: u32 = 0;
        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => heading_level = level as u32,
                Event::End(pulldown_cmark::TagEnd::Heading(_)) => heading_level = 0,
                Event::Start(Tag::CodeBlock(_)) => is_code = true,
                Event::End(pulldown_cmark::TagEnd::CodeBlock) => is_code = false,
                Event::Text(text) => {
                    if is_code {
                        for line in text.lines() {
                            commands.push(Command {
                                raw_string: String::from(line),
                                executable: true,
                            });
                        }
                    } else if heading_level > 0 {
                        for line in text.lines() {
                            commands.push(Command {
                                raw_string: format!(
                                    "{} {}",
                                    "#".repeat(heading_level as usize),
                                    line
                                ),
                                executable: false,
                            });
                        }
                    }
                }
                _ => (),
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

#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod tests {
    use super::*;

    #[test]
    fn test_get_commands_empty_content() {
        let content = String::new();
        let commands = Command::get_commands(content);
        assert!(commands.is_empty());
    }

    #[test]
    fn test_get_commands_code_block_only() {
        let content = r#"
```bash
echo "Hello World"
ls -la
cd /home
```
"#
        .to_string();

        let commands = Command::get_commands(content);
        assert_eq!(commands.len(), 3);

        assert_eq!(commands[0].raw_string, "echo \"Hello World\"");
        assert_eq!(commands[0].executable, true);

        assert_eq!(commands[1].raw_string, "ls -la");
        assert_eq!(commands[1].executable, true);

        assert_eq!(commands[2].raw_string, "cd /home");
        assert_eq!(commands[2].executable, true);
    }

    #[test]
    fn test_get_commands_heading_only() {
        let content = r#"
# Main Title
## Subtitle
### Sub-subtitle
"#
        .to_string();

        let commands = Command::get_commands(content);
        assert_eq!(commands.len(), 3);

        assert_eq!(commands[0].raw_string, "# Main Title");
        assert_eq!(commands[0].executable, false);

        assert_eq!(commands[1].raw_string, "## Subtitle");
        assert_eq!(commands[1].executable, false);

        assert_eq!(commands[2].raw_string, "### Sub-subtitle");
        assert_eq!(commands[2].executable, false);
    }

    #[test]
    fn test_get_commands_mixed_content() {
        let content = r#"
# Installation Guide

This is a guide for installation.

```bash
sudo apt update
sudo apt install git
```

## Configuration

Set up your environment:

```shell
export PATH=/usr/local/bin:$PATH
source ~/.bashrc
```

### Additional Notes

Some final notes here.
"#
        .to_string();

        let commands = Command::get_commands(content);
        assert_eq!(commands.len(), 7);

        // Heading commands
        assert_eq!(commands[0].raw_string, "# Installation Guide");
        assert_eq!(commands[0].executable, false);

        // First code block
        assert_eq!(commands[1].raw_string, "sudo apt update");
        assert_eq!(commands[1].executable, true);

        assert_eq!(commands[2].raw_string, "sudo apt install git");
        assert_eq!(commands[2].executable, true);

        // Second heading
        assert_eq!(commands[3].raw_string, "## Configuration");
        assert_eq!(commands[3].executable, false);

        // Second code block
        assert_eq!(commands[4].raw_string, "export PATH=/usr/local/bin:$PATH");
        assert_eq!(commands[4].executable, true);

        assert_eq!(commands[5].raw_string, "source ~/.bashrc");
        assert_eq!(commands[5].executable, true);

        // Third heading
        assert_eq!(commands[6].raw_string, "### Additional Notes");
        assert_eq!(commands[6].executable, false);
    }

    #[test]
    fn test_get_commands_empty_code_block() {
        let content = r#"
```bash
```
"#
        .to_string();

        let commands = Command::get_commands(content);
        assert!(commands.is_empty());
    }

    #[test]
    fn test_get_commands_empty_lines_in_code_block() {
        let content = r#"
```bash
echo "first"

echo "second"
```
"#
        .to_string();

        let commands = Command::get_commands(content);
        assert_eq!(commands.len(), 3);

        assert_eq!(commands[0].raw_string, "echo \"first\"");
        assert_eq!(commands[0].executable, true);

        assert_eq!(commands[1].raw_string, "");
        assert_eq!(commands[1].executable, true);

        assert_eq!(commands[2].raw_string, "echo \"second\"");
        assert_eq!(commands[2].executable, true);
    }

    #[test]
    fn test_get_commands_multiple_code_blocks() {
        let content = r#"
```bash
ls
```

Some text here.

```python
print("hello")
```
"#
        .to_string();

        let commands = Command::get_commands(content);
        assert_eq!(commands.len(), 2);

        assert_eq!(commands[0].raw_string, "ls");
        assert_eq!(commands[0].executable, true);

        assert_eq!(commands[1].raw_string, "print(\"hello\")");
        assert_eq!(commands[1].executable, true);
    }

    #[test]
    fn test_get_commands_multiline_heading() {
        let content = r#"
# First line
second line
"#
        .to_string();

        let commands = Command::get_commands(content);

        // Adjust the expected values based on the actual behavior of the Markdown parser
        // Headings are treated as single lines, and subsequent text is treated as regular text
        assert_eq!(commands.len(), 1);

        assert_eq!(commands[0].raw_string, "# First line");
        assert_eq!(commands[0].executable, false);
    }

    #[test]
    fn test_command_executable_method() {
        let command1 = Command {
            raw_string: "# echo hello".to_string(),
            executable: true,
        };
        assert_eq!(command1.executable(), "echo hello");

        let command2 = Command {
            raw_string: "$ ls -la".to_string(),
            executable: true,
        };
        assert_eq!(command2.executable(), "ls -la");

        let command3 = Command {
            raw_string: "> git status".to_string(),
            executable: true,
        };
        assert_eq!(command3.executable(), "git status");

        let command4 = Command {
            raw_string: "   ### sudo apt update".to_string(),
            executable: true,
        };
        assert_eq!(command4.executable(), "sudo apt update");
    }

    #[test]
    fn test_command_display() {
        let command = Command {
            raw_string: "echo hello".to_string(),
            executable: true,
        };
        assert_eq!(format!("{}", command), "echo hello");
    }
}
