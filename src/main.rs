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
    #[tabled(rename = "Distribution")]
    bar: String,
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

fn create_bar(percentage: f64, max_width: usize) -> String {
    let filled = ((percentage / 100.0) * max_width as f64) as usize;
    let empty = max_width - filled;
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

fn color_percentage(percentage: f64, text: String) -> String {
    if percentage >= 10.0 {
        text.green().to_string()
    } else if percentage >= 5.0 {
        text.yellow().to_string()
    } else {
        text.normal().to_string()
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let exts: HashSet<&str> = if let Some(ext_string) = &cli.exts {
        ext_string.split(',').collect()
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

            // Calculate summary statistics
            let total_symbols: usize = read_result.symbol_counts.values().sum();
            let unique_symbols = read_result.symbol_counts.len();
            let total_files =
                read_result.files_read + read_result.files_skipped + read_result.files_failed;
            let files_processed_percent = if total_files > 0 {
                (read_result.files_read as f64 / total_files as f64) * 100.0
            } else {
                0.0
            };

            // Print summary section
            println!("\n{}", "Summary:".bold());
            println!(
                "  Total tracked symbols: {}",
                total_symbols.to_string().cyan()
            );
            println!("  Unique symbols: {}", unique_symbols.to_string().cyan());
            println!(
                "  Files processed: {} read ({:.1}%), {} skipped, {} failed\n",
                read_result.files_read.to_string().green(),
                files_processed_percent,
                read_result.files_skipped.to_string().yellow(),
                read_result.files_failed.to_string().red()
            );

            // Create table rows with bars and colored percentages
            let bar_width = 25;
            let rows: Vec<Row> = sorted_percentages
                .into_iter()
                .map(|(char, percentage)| {
                    let percent_text = format!("{percentage:.2}%");
                    Row {
                        symbol: char.to_string(),
                        percent: color_percentage(percentage, percent_text),
                        bar: create_bar(percentage, bar_width),
                    }
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
