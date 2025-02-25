use clap::{Parser, Subcommand};

mod git_operations;
mod commit;
mod branch;
mod checkout;
mod config;

#[derive(Parser)]
#[command(name = "cmt", version = "1.0", about = "Commit management tool")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Commit,
    Checkout,
    Branch,
    Ignore,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Commit) => {
            if let Err(err) = commit::run_commit() {
                eprintln!("❌ Error: {}", err);
                std::process::exit(1);
            }
        }
        Some(Commands::Branch) => {
            if let Err(err) = branch::run_branch() {
                eprintln!("❌ Error: {}", err);
                std::process::exit(1);
            }
        }
        Some(Commands::Checkout) => {
            if let Err(err) = checkout::run_checkout() {
                eprintln!("❌ Error: {}", err);
                std::process::exit(1);
            }
        }
        Some(Commands::Ignore) => {
            println!("Ignore logic to implement later");
        }
        None => {
            println!("Default logic to implement later");
        }
    }
}