use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ArgType {
    Int,
    Float,
    String,
    Bool,
    #[serde(alias = "path")]
    Filepath,
}

impl ArgType {
    /// Detailed human-readable type description
    pub fn verbose_description(&self) -> &'static str {
        match self {
            ArgType::Int => "Integer (e.g., 1, 42)",
            ArgType::Float => "Floating-point number (e.g., 3.14)",
            ArgType::String => "Text string",
            ArgType::Bool => "Boolean value (true/false)",
            ArgType::Filepath => "File or Directory Path"
        }
    }

    /// Placeholder label used in usage strings (e.g., <INT: N>)
    pub fn value_name(&self) -> &'static str {
        match self {
            ArgType::Int => "INT",
            ArgType::Float => "FLOAT",
            ArgType::String => "STR",
            ArgType::Bool => "BOOL",
            ArgType::Filepath => "PATH"
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ArgConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: ArgType,
    #[serde(default)]
    pub required: bool,
    /// Optional field to allow custom descriptions in generators.json
    pub description: Option<String>,
    pub long: Option<String>,
    pub short: Option<char>
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeneratorConfig {
    pub script_path: Option<PathBuf>,
    pub cnf_path: Option<PathBuf>,
    pub solver_out_path: Option<PathBuf>,
    pub args: Vec<ArgConfig>,
}

pub type Config = HashMap<String, GeneratorConfig>;

pub fn load_config(path: &str) -> Config {
    let content = std::fs::read_to_string(path).expect("Failed to read config file");
    serde_json::from_str(&content).expect("Failed to parse JSON config")
}