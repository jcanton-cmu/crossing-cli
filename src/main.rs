mod commands;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "Crossing-Numbers CLI",
    about = "CNF generator and SAT solver runner"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate CNF files and optionally run SAT solvers
    Run(commands::run::RunArgs),

    /// Download and build Kissat and CaDiCaL executables
    Build(commands::build::BuildArgs),

    /// Remove ./cnf, ./out, and solver repos
    Clean(commands::clean::CleanArgs),
}

fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Run(args) => commands::run::execute(args),
        Commands::Build(args) => commands::build::execute(args),
        Commands::Clean(args) => commands::clean::execute(args),
    }
}