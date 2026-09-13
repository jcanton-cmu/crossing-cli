mod commands;
mod config;
mod utils;

use clap::{value_parser, Arg, Command, CommandFactory, FromArgMatches, Parser, Subcommand};
use config::{load_config, ArgType, Config};

#[derive(Parser)]
#[command(
    name = "hill",
    about = "CNF generator and SAT solver runner",
    disable_help_subcommand = true,
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate CNF files and optionally run SAT solvers
    Run(commands::run::RunArgs),

    /// Download and build Kissat and CaDiCaL executables
    Build(commands::build::BuildArgs),

    /// Remove ./cnf, ./out, and solver repos
    Clean(commands::clean::CleanArgs),
}

fn main() {
    let config: Config = load_config("generators.json");

    let mut cmd = Cli::command();

    // Take ownership of 'run_cmd' to mutate it in-place without cloning
    if let Some(run_cmd) = cmd.find_subcommand_mut("run") {
        let mut run = std::mem::take(run_cmd).arg_required_else_help(true).subcommand_required(true);

        for (gen_name, generator) in &config {
            let gen_name_st: &'static str = Box::leak(gen_name.clone().into_boxed_str());

            let mut gen_subcmd = Command::new(gen_name_st)
                .arg_required_else_help(true)
                .about(format!("Run {gen_name} CNF generator"));

            for arg in &generator.args {
                let arg_name: &'static str = Box::leak(arg.name.clone().into_boxed_str());
                let arg_type: &'static ArgType = Box::leak(Box::new(arg.arg_type.clone()));
                let value_name: &'static str = Box::leak(
                    format!("{}:{}", arg_type.value_name(), arg_name).into_boxed_str()
                );

                let help_text = match &arg.description {
                    Some(desc) => format!("{desc} [{}]", arg.arg_type.verbose_description()),
                    None => format!("Type: {}", arg_type.verbose_description()),
                };

                let mut clap_arg = Arg::new(arg_name)
                    .value_name(value_name)
                    .help(help_text);

                clap_arg = match arg.arg_type {
                    ArgType::Int => clap_arg.value_parser(value_parser!(i64)),
                    ArgType::Float => clap_arg.value_parser(value_parser!(f64)),
                    ArgType::String => clap_arg.value_parser(value_parser!(String)),
                    ArgType::Bool => clap_arg.value_parser(value_parser!(bool)),
                };

                if arg.required {
                    clap_arg = clap_arg.required(true);
                }

                gen_subcmd = gen_subcmd.arg(clap_arg);
            }

            run = run.subcommand(gen_subcmd);
        }

        // Place the fully built subcommand back into cmd
        *run_cmd = run;
    }

    let matches = cmd.get_matches();

    match Commands::from_arg_matches(&matches) {
        Ok(Commands::Run(args)) => {
            let run_matches = matches.subcommand_matches("run").unwrap();
            commands::run::execute(args, run_matches, &config);
        }
        Ok(Commands::Build(args)) => commands::build::execute(args),
        Ok(Commands::Clean(args)) => commands::clean::execute(args),
        Err(err) => err.exit(),
    }
}