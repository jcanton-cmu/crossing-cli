use clap::Args;
use crate::utils::{ensure_cadical, ensure_kissat};

#[derive(Args, Debug)]
pub struct BuildArgs {
    /// Build Kissat
    #[arg(short = 'k', long)]
    pub kissat: bool,

    /// Build CaDiCaL
    #[arg(short = 'c', long)]
    pub cadical: bool,
}

pub fn execute(mut args: BuildArgs) {
    if !args.kissat && !args.cadical {
        args.kissat = true;
        args.cadical = true;
    }

    println!("Pre-building solvers...");
    if args.kissat {
        ensure_kissat();
        println!("Kissat build complete.");
    }
    if args.cadical {
        ensure_cadical();
        println!("CaDiCaL build complete.");
    }
    println!("Build complete.");
}