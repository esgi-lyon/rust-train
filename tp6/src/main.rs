use clap::Parser;
use std::env;

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn by_type_args(pattern: String, path: String) -> Cli {
    return Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };
}

fn manual_args(mut args: env::Args) -> Cli {
    let pattern = args.nth(1).expect("no pattern given");
    let path = args.nth(2).expect("no path given");

    return by_type_args(pattern, path);
}

fn search_lines(cli: Cli) {
    let content = std::fs::read_to_string(&cli.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&cli.pattern) {
            println!("{}", line);
        }
    }
}

fn main() {
    let args = env::args();

    let cli_manual = manual_args(args);

    search_lines(cli_manual);

    // Second way

    let args = Cli::parse();

    search_lines(args);
}
