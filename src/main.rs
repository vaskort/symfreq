use std::collections::HashSet;
use clap::Parser;
use std::process::ExitCode;
use tabled::{Table, Tabled};
use symfreq::{count_percentages, count_symbols, read_path, sorted_percentages, DEFAULT_EXTENSIONS};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    path: String,
    #[arg(short, long)]
    exts: Option<String>, // User provides comma-separated like "rs,js,ts"
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
    let exts: HashSet<&str> = if let Some(ext_string) = &cli.exts {
        ext_string.split(",").collect()
    } else {
        DEFAULT_EXTENSIONS.iter().copied().collect()
    };

    match read_path(&cli.path, &exts) {
        Ok(read_result) => {
            let counts = count_symbols(&read_result.content);
            let count_percentages = count_percentages(&counts);
            let sorted_percentages = sorted_percentages(&count_percentages);

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
