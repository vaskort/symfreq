use clap::Parser;
use std::fs;
use std::process::ExitCode;
use tabled::{Table, Tabled};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    path: String,
}

#[derive(Tabled)]
struct Row {
    #[tabled(rename = "Symbol")]
    symbol: String,
    #[tabled(rename = "Percent")]
    percent: String,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let exts: HashSet<&str> = ["rs", "js", "jsx", "ts", "tsx"]
        .iter()
        .copied()
        .collect();

    match fs::read_to_string(&cli.path) {
        Ok(content) => {
            let counts = symfreq::count_symbols(&content);
            let count_percentages = symfreq::count_percentages(&counts);
            let sorted_percentages = symfreq::sorted_percentages(&count_percentages);

            let rows: Vec<Row> = sorted_percentages
                .into_iter()
                .map(|(char, percentage)| Row {
                    symbol: char.to_string(),
                    percent: format!("{percentage:.2}%"),
                })
                .collect();

            println!("{}", Table::new(rows));
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Failed to read {}: {e}", cli.path);
            ExitCode::from(1)
        }
    }
}
