use std::process::Command;
use structopt::StructOpt;
use regex::Regex;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    filename: std::path::PathBuf
}

fn main() {
    println!("Hello, i-read-u!");

    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.filename).expect("No such file or directory");

    // FIXME: 選択可能にする
    let mut cmd_str = String::from("ls -l -a");
    let cmd_pattern = Regex::new(r"^\s*[#>$]+\s*").unwrap();
    for line in content.lines() {
        if cmd_pattern.is_match(line) {
            cmd_str = cmd_pattern.replace(line, "").into_owned();
        }
    }

    println!("command: {}", cmd_str);

    let mut result = Command::new("sh")
        .arg("-c")
        .arg(cmd_str)
        .spawn()
        .unwrap();
    result.wait();
}
