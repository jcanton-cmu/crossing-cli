use clap::Args;
use std::fs;

#[derive(Args, Debug)]
pub struct CleanArgs {
    /// Remove only ./out directory
    #[arg(short = 'o', long)]
    pub out: bool,

    /// Remove only ./cnf directory
    #[arg(short = 'c', long)]
    pub cnf: bool,

    /// Remove only ./kissat and ./cadical directories
    #[arg(short = 'd', long)]
    pub dependencies: bool,
}

pub fn execute(mut args: CleanArgs) {
    if !args.out && !args.cnf && !args.dependencies {
        args.out = true;
        args.cnf = true;
    }

    println!("Cleaning generated files...");
    if args.out {
        let _ = fs::remove_dir_all("./out");
        println!("Removed ./out directory.");
    }
    if args.cnf {
        let _ = fs::remove_dir_all("./cnf");
        println!("Removed ./cnf directory.");
    }
    if args.dependencies {
        let _ = fs::remove_dir_all("./kissat");
        let _ = fs::remove_dir_all("./cadical");
        println!("Removed ./kissat and ./cadical directories.");
    }
    println!("Clean complete.");
}