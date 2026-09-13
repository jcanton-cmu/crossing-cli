use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::env::consts::EXE_SUFFIX;

pub fn get_local_solver_path(solver_name: &str) -> PathBuf {
    PathBuf::from(format!("./{solver_name}/build/{solver_name}{EXE_SUFFIX}"))
}

pub fn ensure_tools(tools: &[&str]) {
    for tool in tools {
        let status = Command::new(tool)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        if status.is_err() || !status.unwrap().success() {
            eprintln!("Error: Required tool '{tool}' is not installed or not in PATH.");
            std::process::exit(1);
        }
    }
}

pub fn command_exists(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn ensure_kissat() {
    let local_bin = get_local_solver_path("kissat");
    if !command_exists("kissat") && !local_bin.exists() {
        println!("Kissat executable not found. Downloading and compiling...");
        ensure_tools(&["git", "make", "python3"]);

        Command::new("git")
            .args(["clone", "https://github.com/arminbiere/kissat.git"])
            .status()
            .expect("Failed to clone kissat");

        Command::new("./configure")
            .current_dir("./kissat")
            .status()
            .expect("Failed to configure kissat");

        Command::new("make")
            .current_dir("./kissat")
            .status()
            .expect("Failed to make kissat");
    }
}

pub fn ensure_cadical() {
    let local_bin = get_local_solver_path("cadical");
    if !command_exists("cadical") && !local_bin.exists() {
        println!("CaDiCaL executable not found. Downloading and compiling...");
        ensure_tools(&["git", "make", "python3"]);

        Command::new("git")
            .args(["clone", "https://github.com/arminbiere/cadical.git"])
            .status()
            .expect("Failed to clone cadical");

        Command::new("./configure")
            .current_dir("./cadical")
            .status()
            .expect("Failed to configure cadical");

        Command::new("make")
            .current_dir("./cadical")
            .status()
            .expect("Failed to make cadical");
    }
}

pub fn get_solver_bin<'a>(cmd_name: &'a str, fallback_path: &'a str) -> &'a str {
    if command_exists(cmd_name) {
        cmd_name
    } else {
        fallback_path
    }
}

pub fn run_python_builder(script: &Path, args: &[String], output_path: &Path) {
    let out_file = File::create(output_path).expect("Failed to create CNF file");
    
    Command::new("python3")
        .arg(script)
        .args(args)
        .stdout(out_file)
        .status()
        .expect("Failed to execute python builder");
}

pub fn run_solver(solver_name: &str, solver_bin: &str, cnf_file: &Path, output_path: &Path) {
    let out_file_path = output_path.to_string_lossy();
    let output = Command::new(solver_bin)
        .arg(cnf_file)
        .output()
        .expect("Failed to execute solver");

    let mut log_content = output.stdout.clone();
    log_content.extend_from_slice(&output.stderr);
    fs::write(output_path, log_content).expect("Failed to create log file");

    let content = String::from_utf8_lossy(&output.stdout);
    let sat_status = content
        .lines()
        .find(|line| line.starts_with("s "))
        .unwrap_or("s UNKNOWN");

    let file_name = cnf_file.file_name().unwrap().to_str().unwrap();
    println!("[{solver_name}] Output for {file_name}: {sat_status} ({out_file_path})");
}