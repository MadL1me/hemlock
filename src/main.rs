mod remote_sources;
mod vendors;
mod cli_progress;

use std::{fs::{self}, path::PathBuf, string};
use colored::Colorize;
use std::fmt::format;
use std::path::Path;
use clap::Parser;
use serde::{Deserialize, Serialize};
use crate::vendors::AnyError;
use crate::vendors::remote_git_vendor::{get_git_vendor_source, vendor};
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser, Debug)]
#[command(name = "hemlock")]
#[command(version = "0.0.1")]
#[command(about = "CLI tool for vendoring remote files")]
#[command(long_about = None)]
struct CliArgs {
    #[arg(short, long, value_name = "FILE", default_value = "hemlock.yaml")]
    config: Option<PathBuf>,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct YamlConfigV1 {
    version: String,
    vendor_dir: Option<String>,
    default_branch: Option<String>,

    #[serde(default)]
    local_deps: Vec<String>,

    #[serde(default)]
    external_deps: Vec<ExternalDep>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExternalDep {
    pub import: String,

    #[serde(default)]
    pub tag: Option<String>,

    #[serde(default)]
    pub target_file: Option<String>,

    #[serde(default)]
    pub target_dir: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VendoringOptions {
    vendor_dir: String,
    default_branch: String,
}

fn main() -> Result<(), AnyError> {
    let args = CliArgs::parse();

    // Check if the config path was provided
    let config_path = args.config.ok_or("No config path provided")?;

    // Read the configuration file
    let file_res = fs::read_to_string(config_path.clone())
        .map_err(|err| format!("Failed to read the config file: {}", err))?;

    // Parse the YAML content
    let mut config: YamlConfigV1 = serde_yaml::from_str(&file_res)
        .map_err(|err| format!("Failed to parse config: {}", err))?;

    // Use the parsed config (here, just printing it)
    if args.verbose {
        println!("Verbose mode enabled");
    }

    if config.default_branch.is_none() {
        config.default_branch = Some(String::from("master"));
    }

    if config.vendor_dir.is_none() {
        config.vendor_dir = Some(String::from("vendor"));
    }

    //println!("Config loaded successfully: {:?}", config);

    // Extract default branch as &str
    let default_branch = config.default_branch.as_deref().unwrap();

    let vendoringOpts = VendoringOptions {
        vendor_dir: config.vendor_dir.unwrap(),
        default_branch: default_branch.to_string(),
    };

    println!("{} {}", "Starting vendoring process for configuration in".bold(),
             config_path.clone().to_str().unwrap().bold().blue());

    let mut sb = String::new();

    sb.push_str("");


    for dep in config.external_deps {
        let source = get_git_vendor_source(dep.clone(), default_branch).unwrap();

        println!("---");
        println!("{} {}", "Starting vendoring url:".bold(),
                 dep.import.clone().bold().blue());

        vendor(source.clone(), vendoringOpts.clone());
    }

    println!();
    println!();
    println!("{}", "Vendoring completed successfully!".bold().green());

    Ok(())
}

//
// // All cases needs to be addressed:
//
// // https://github.com/MadL1me/RhythmGE/blob/68f7379aa960a365d7eb61577d536307334a4e2e/src/index.scss
// // http://github.com/MadL1me/RhythmGE/blob/68f7379aa960a365d7eb61577d536307334a4e2e/src/index.scss
// // github.com/MadL1me/RhythmGE/src/index.scss
// // github.com/MadL1me/RhythmGE/src/index.scss@master
// // github.com/MadL1me/RhythmGE/src/index.scss@68f7379aa960a365d7eb61577d536307334a4e2e
// // https://github.com/MadL1me/RhythmGE/src/index.scss@master
// // http://github.com/MadL1me/RhythmGE/src/index.scss
// // http://github.com/MadL1me/RhythmGE/src/index.scss@68f7379aa960a365d7eb61577d536307334a4e2e