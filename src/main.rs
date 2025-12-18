use clap::Parser;

/// A small example CLI for the template.
#[derive(Parser)]
struct Cli {
    /// Name to greet
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let name = cli.name.unwrap_or_else(|| "world".into());
    println!("Hello, {}!", name);
}
