use std::process::Command;
use structopt::StructOpt;
use regex::Regex;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    filename: std::path::PathBuf
}

fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.filename).expect("No such file or directory");

    let mut commands: Vec<String> = Vec::new();
    let cmd_pattern = Regex::new(r"^\s*[#>$]+\s*").unwrap();
    for line in content.lines() {
        if cmd_pattern.is_match(line) {
            commands.push(cmd_pattern.replace(line, "").into_owned());
        }
    }

    let select = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("choice command")
        .default(0)
        .items(&commands)
        .interact()
        .unwrap();

    println!("execute command: {}", commands[select]);

    let mut result = Command::new("sh")
        .arg("-c")
        .arg(commands[select].to_string())
        .spawn()
        .unwrap();
    result.wait();
}
