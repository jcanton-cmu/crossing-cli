use clap::{Args, ValueEnum};
use std::fs;
use std::path::PathBuf;

use crate::utils::{
    ensure_cadical, ensure_kissat, get_solver_bin, run_python_builder, run_solver,
};

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum IsolateTarget {
    Sympy,
    Mackey,
}

#[derive(Args, Debug)]
pub struct RunArgs {
    /// Solve using Kissat
    #[arg(short = 'k', long)]
    pub kissat: bool,

    /// Solve using CaDiCaL
    #[arg(short = 'c', long)]
    pub cadical: bool,

    /// Run only the specified python CNF generator
    #[arg(short = 'i', long, value_enum)]
    pub isolate: Option<IsolateTarget>,

    /// K_n
    pub n: u64,

    /// Maximum number of crossings
    pub k: u64,
}

pub fn execute(args: RunArgs) {
    fs::create_dir_all("./cnf").expect("Failed to create ./cnf directory");
    fs::create_dir_all("./out").expect("Failed to create ./out directory");

    let py_sympy = "./builders/sympy_cnf.py";
    let py_mackey = "./builders/hill_cnf.py";
    let cnf_sympy = PathBuf::from("./cnf/sympy.cnf");
    let cnf_mackey = PathBuf::from("./cnf/mackey.cnf");

    let mut cnf_targets = Vec::new();

    if args.isolate.is_none() || args.isolate == Some(IsolateTarget::Sympy) {
        println!("[Running] sympy CNF generator");
        run_python_builder(py_sympy, args.n, args.k, &cnf_sympy);
        cnf_targets.push(cnf_sympy);
    }

    if args.isolate.is_none() || args.isolate == Some(IsolateTarget::Mackey) {
        println!("[Running] mackey CNF generator");
        run_python_builder(py_mackey, args.n, args.k, &cnf_mackey);
        cnf_targets.push(cnf_mackey);
    }

    if args.kissat {
        ensure_kissat();
        let kissat_bin = get_solver_bin("kissat", "./kissat/build/kissat");
        for cnf_file in &cnf_targets {
            run_solver("kissat", &kissat_bin, cnf_file);
        }
    }

    if args.cadical {
        ensure_cadical();
        let cadical_bin = get_solver_bin("cadical", "./cadical/build/cadical");
        for cnf_file in &cnf_targets {
            run_solver("cadical", &cadical_bin, cnf_file);
        }
    }
}