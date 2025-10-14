use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    path: String,
}

fn main() {
    let _cli = Cli::parse();
    println!("symfreq-cli is wired up. Next: read file, run symfreq, print table.");
}
