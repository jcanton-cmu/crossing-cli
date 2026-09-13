use clap::{ArgMatches, Args};
use std::fs;
use std::path::PathBuf;

use crate::config::{Config, ArgType};
use crate::utils::{
    ensure_cadical, ensure_kissat, get_solver_bin, run_python_builder, run_solver,
};

#[derive(Args, Debug)]
pub struct RunArgs {
    /// Solve using Kissat
    #[arg(short = 'k', long)]
    pub kissat: bool,

    /// Solve using CaDiCaL
    #[arg(short = 'c', long)]
    pub cadical: bool,
}

pub fn execute(args: RunArgs, run_matches: &ArgMatches, config: &Config) {
    fs::create_dir_all("./cnf").expect("Failed to create ./cnf directory");
    fs::create_dir_all("./out").expect("Failed to create ./out directory");

    // Extract the invoked subcommand name and its argument matches
    let (gen_name, gen_matches) = match run_matches.subcommand() {
        Some((name, matches)) => (name, matches),
        None => return,
    };

    let gen_config = match config.get(gen_name) {
        Some(cfg) => cfg,
        None => return,
    };

    println!("[Running] generator: {gen_name}");

    let mut script_args: Vec<String> = Vec::new();
    for arg_spec in &gen_config.args {
        let is_named = arg_spec.long.is_some() || arg_spec.short.is_some();

        if is_named {
            let flag_prefix = if let Some(long) = &arg_spec.long {
                format!("--{long}")
            } else {
                format!("-{}", arg_spec.short.unwrap())
            };

            if arg_spec.arg_type == ArgType::Bool {
                if gen_matches.get_flag(&arg_spec.name) {
                    script_args.push(flag_prefix);
                }
            } else if let Some(raw_val) = gen_matches.get_raw(&arg_spec.name) {
                for val in raw_val {
                    script_args.push(flag_prefix.clone());
                    script_args.push(val.to_string_lossy().into_owned());
                }
            }
        } else if let Some(raw_val) = gen_matches.get_raw(&arg_spec.name) {
            for val in raw_val {
                script_args.push(val.to_string_lossy().into_owned());
            }
        }
    }

    let cnf_path = gen_config
        .cnf_path
        .clone()
        .unwrap_or_else(|| PathBuf::from(format!("./cnf/{gen_name}.cnf")));

    let out_path = gen_config
        .solver_out_path
        .clone()
        .unwrap_or_else(|| PathBuf::from(format!("./out/{gen_name}.out")));

    if let Some(script) = &gen_config.script_path {
        run_python_builder(script, &script_args, &cnf_path);
    }

    if args.kissat {
        ensure_kissat();
        let kissat_bin = get_solver_bin("kissat", "./kissat/build/kissat");
        run_solver("kissat", &kissat_bin, &cnf_path, &out_path);
    }

    if args.cadical {
        ensure_cadical();
        let cadical_bin = get_solver_bin("cadical", "./cadical/build/cadical");
        run_solver("cadical", &cadical_bin, &cnf_path, &out_path);
    }
}