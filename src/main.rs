use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashSet;
use std::process::ExitCode;
use symfreq::{DEFAULT_EXTENSIONS, count_percentages, read_path, sorted_percentages};
use tabled::{Table, Tabled};

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

fn start_spinner() -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message("Analyzing files...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    spinner
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let exts: HashSet<&str> = if let Some(ext_string) = &cli.exts {
        ext_string.split(",").collect()
    } else {
        DEFAULT_EXTENSIONS.iter().copied().collect()
    };

    let spinner = start_spinner();

    let result = read_path(&cli.path, &exts);
    spinner.finish_and_clear();

    match result {
        Ok(read_result) => {
            let count_percentages = count_percentages(&read_result.symbol_counts);
            let sorted_percentages = sorted_percentages(&count_percentages);

            let rows: Vec<Row> = sorted_percentages
                .into_iter()
                .map(|(char, percentage)| Row {
                    symbol: char.to_string(),
                    percent: format!("{percentage:.2}%"),
                })
                .collect();
            println!(
                "\nFiles processed: {} read, {} skipped, {} failed\n",
                &read_result.files_read.to_string().green(),
                &read_result.files_skipped.to_string().yellow(),
                &read_result.files_failed.to_string().red()
            );
            println!("{}", Table::new(rows));
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Failed to read {}: {e}", cli.path);
            ExitCode::from(1)
        }
    }
}
