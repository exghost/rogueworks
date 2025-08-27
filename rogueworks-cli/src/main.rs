use clap::Parser;

/// Simple CLI (placeholder) to print a banner and exit.
#[derive(Parser, Debug)]
#[command(name = "rogueworks-cli")]
#[command(about = "Rogueworks CLI utilities", long_about = None)]
struct Cli {}

fn main() {
    let _cli = Cli::parse();
    println!("Rogueworks CLI (stub)");
}
