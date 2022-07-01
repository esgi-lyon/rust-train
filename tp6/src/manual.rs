use std::env;

fn by_type_args(pattern: String, path: String) -> Cli {
    return Cli {
        pattern,
        path: std::path::PathBuf::from(path),
    };
}

fn manual_args(mut args: env::Args) -> Cli {
    let pattern = args.nth(1).expect("no pattern given");
    let path = args.nth(2).expect("no path given");

    return by_type_args(pattern, path);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args();

    let cli_manual = manual_args(args);

    return lib_cli::search_lines(cli_manual);
}
