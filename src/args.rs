use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(default_value = "README.md")]
    filename: std::path::PathBuf,
}

pub fn get_content() -> String {
    let args = Cli::parse();

    std::fs::read_to_string(args.filename).expect("No such file or directory")
}
