mod lib_cli;

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = lib_cli::Cli::parse();

    return lib_cli::search_lines(args);
}
