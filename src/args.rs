use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str), default_value("README.md"))]
    filename: std::path::PathBuf
}

pub fn get_content() -> String {
    let args = Cli::from_args();

    std::fs::read_to_string(args.filename)
        .expect("No such file or directory")
}
