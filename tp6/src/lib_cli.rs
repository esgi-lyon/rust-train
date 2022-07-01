use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

pub fn search_lines(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let read_file_result = std::fs::read_to_string(&cli.path);

    let content = match read_file_result {
        Ok(file_content) => file_content,
        Err(error) => {
            return Err(error.into());
        }
    };

    for line in content.lines() {
        if line.contains(&cli.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
