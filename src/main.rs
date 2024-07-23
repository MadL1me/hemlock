use std::process::Command;
use std::io::{self, Write};
use std::str;

fn run_command(cmd: &mut Command) -> io::Result<()> {
    let output = cmd.output()?;
    if !output.status.success() {
        eprintln!("Command failed with status: {:?}", output.status);
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        Err(io::Error::new(io::ErrorKind::Other, "Command execution failed"))
    } else {
        let s = String::from_utf8(output.stdout).expect("NOT VALID UTF8");

        println!("{:?}", s);
        Ok(())
    }
}

fn fetch_single_file(repo_url: &str, branch: &str, file_path: &str, target_dir: &str) -> io::Result<()> {
    // Initialize a new empty repository in the target directory
    run_command(Command::new("git").arg("init").arg(target_dir))?;

    // Change to the target directory
    std::env::set_current_dir(target_dir)?;

    // Add a remote pointing to the repository URL
    run_command(Command::new("git").arg("remote").arg("add").arg("origin").arg(repo_url))?;

    // Set up sparse checkout
    run_command(Command::new("git").arg("config").arg("core.sparseCheckout").arg("true"))?;

    // Write the file path to the sparse-checkout file
    {
        use std::fs::OpenOptions;

        let sparse_checkout_path = ".git/info/sparse-checkout";
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(sparse_checkout_path)?;

        writeln!(file, "{}", file_path)?;
    }

    // Fetch the specified branch
    run_command(Command::new("git").arg("fetch").arg("origin").arg(branch))?;

    // Checkout the specified branch
    run_command(Command::new("git").arg("checkout").arg(branch))?;

    Ok(())
}

fn main() -> io::Result<()> {
    let repo_url = "git@git.tech.manee.io:antifraud/antifraud.event.service.net.v1.git";
    let branch = "main"; // Make sure this is the correct branch name
    let file_path = "src/Event.Api/Contracts/**.yaml";
    let target_dir = "repo";

    fetch_single_file(repo_url, branch, file_path, target_dir)?;

    println!("File {} has been checked out to {}", file_path, target_dir);
    Ok(())
}

mod errors;
mod remote_sources;
mod vendors;

use std::{fs::{self, File}, path::PathBuf};
use clap::Parser;
use serde::{Deserialize, Serialize};
use vendors::*;

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

#[derive(Debug, Serialize, Deserialize)]
struct ExternalDep {
    import: String,

    #[serde(default)]
    tag: Option<String>,

    #[serde(default)]
    file: Option<String>,
}

#[derive(Debug, Clone)]
struct VendorSource {
    source: String,
    version: String,
    file_name: String,
}

type AnyError = Box<dyn std::error::Error>;
//
// fn main() -> Result<(), AnyError> {
//     let args = CliArgs::parse();
//
//     // Check if the config path was provided
//     let config_path = args.config.ok_or("No config path provided")?;
//
//     // Read the configuration file
//     let file_res = fs::read_to_string(config_path)
//         .map_err(|err| format!("Failed to read the config file: {}", err))?;
//
//     // Parse the YAML content
//     let mut config: YamlConfigV1 = serde_yaml::from_str(&file_res)
//         .map_err(|err| format!("Failed to parse config: {}", err))?;
//
//     // Use the parsed config (here, just printing it)
//     if args.verbose {
//         println!("Verbose mode enabled");
//     }
//
//     if config.default_branch.is_none() {
//         config.default_branch = Some(String::from("master"));
//     }
//
//     if config.vendor_dir.is_none() {
//         config.vendor_dir = Some(String::from("vendor"));
//     }
//
//     //println!("Config loaded successfully: {:?}", config);
//
//     // Extract default branch as &str
//     let default_branch = config.default_branch.as_deref().unwrap_or("master");
//
//     for dep in config.external_deps {
//         let source = map_vendor_source(dep, default_branch);
//
//         vendor_path(source.clone());
//
//         if args.verbose {
//             println!("external dep parsed successfully: {:?}", source);
//         }
//     }
//
//     //
//     // let vendor = FileVendor::vendor_path(&config.vendor_dir.unwrap());
//     //
//     // for remote_dep in config.external_deps {
//     //     vendor_remote(&remote_dep[..], &config.vendor_dir.as_ref().unwrap()[..], args.verbose)?;
//     // }
//
//     // for local_dep in config.local_deps {
//     //     vendor_local(&local_dep[..])?;
//     // }
//
//     Ok(())
// }
//
// fn map_vendor_source(dep: ExternalDep, default_branch: &str) -> VendorSource {
//     let file_name = dep.file.clone().unwrap_or_else(|| {
//         dep.import
//             .split('/')
//             .last()
//             .unwrap_or("")
//             .split('@')
//             .next()
//             .unwrap_or("")
//             .to_string()
//     });
//
//     // Extract the version from `import` if `tag` is None
//     let (version, source) = match &dep.tag {
//         Some(tag) => (tag.clone(), dep.import.replace(&format!("@{}", tag), "")),
//         None => {
//             if let Some(index) = dep.import.rfind('@') {
//                 let version = dep.import[(index + 1)..].to_string();
//                 let source = dep.import[..index].to_string();
//                 (version, source)
//             } else {
//                 (default_branch.to_string(), dep.import.clone())
//             }
//         }
//     };
//
//     VendorSource {
//         source,
//         version,
//         file_name,
//     }
// }
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