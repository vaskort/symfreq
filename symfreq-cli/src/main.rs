use std::fs;
use std::process::ExitCode;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    path: String,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match fs::read_to_string(&cli.path) {
        Ok(content) => {
            let counts = symfreq::count_symbols(&content);
            let count_percentages = symfreq::count_percentages(&counts);
            let sorted_percentages = symfreq::sorted_percentages(&count_percentages);
            
            println!("{:?}", sorted_percentages);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Failed to read {}: {e}", cli.path);
            ExitCode::from(1)
        }
    }
}
