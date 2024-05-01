mod errors;
use crate::errors::ErrorBase;

use std::{fs::{self, File}, io, path::PathBuf};
use reqwest::blocking::Client;
use clap::Parser;
use serde::{Deserialize, Serialize};
use regex::Regex;
use std::path::Path;

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
    
    #[serde(default)]
    local_deps: Vec<String>,
    
    #[serde(default)]
    external_deps: Vec<String>,
}

type AnyError = Box<dyn std::error::Error>;

fn main() -> Result<(), AnyError> {
    let args = CliArgs::parse();

    // Check if the config path was provided
    let config_path = args.config.ok_or("No config path provided")?;

    // Read the configuration file
    let file_res = fs::read_to_string(config_path)
        .map_err(|err| format!("Failed to read the config file: {}", err))?;

    // Parse the YAML content
    let mut config: YamlConfigV1 = serde_yaml::from_str(&file_res)
        .map_err(|err| format!("Failed to parse config: {}", err))?;

    // Use the parsed config (here, just printing it)
    if args.verbose {
        println!("Verbose mode enabled");
    }

    if config.vendor_dir.is_none() {
        config.vendor_dir = Some(String::from("vendor_dir"));
    }

    println!("Config loaded successfully: {:?}", config);

    for remote_dep in config.external_deps {
        download_locally(&remote_dep[..], &config.vendor_dir.as_ref().unwrap()[..], args.verbose)?;
    }

    Ok(())
}

fn download_locally(url: &str, dir: &str, verbose: bool) -> Result<&'static str, AnyError> {
    let source = RemoteFileSource::from_url(url)?;

    let response = Client::new().get(source.download_url).send()?;

    let filename = url.split('/').last().ok_or("Error")?;

    if response.status().is_success() {
        let path = format!("{dir}/{filename}");
        let path_obj = Path::new(&path);

        if let Some(parent) = path_obj.parent() {
            fs::create_dir_all(parent)?;
        } 

        let mut file = File::create(&path)?;
        let mut content = io::Cursor::new(response.bytes()?);
        io::copy(&mut content, &mut file)?;
        println!("File downloaded successfully.");
    } else {
        println!("Failed to download file: HTTP {}", response.status());
    }

    Ok("successfully donwloaded a file")
}

pub struct RemoteFileSource {
    original_url: String,
    source_origin: SourceOrigin,

    user: String,
    repo: String,
    commit_or_branch: String,
    download_url: String,
}   

impl RemoteFileSource {
    pub fn from_url(url: &str) -> Result<RemoteFileSource, AnyError> {
        let source_origin = SourceOrigin::source_matcher(url);
        let result: Result<RemoteFileSource, AnyError>;
        
        match source_origin {
            SourceOrigin::GitHub => return RemoteFileSource::github_to_raw(url),
            SourceOrigin::GitLab => todo!(),
            SourceOrigin::Unknown => todo!(),
        };
    }
    
    fn github_to_raw(url: &str) -> Result<RemoteFileSource, AnyError> {
        let re = Regex::new(r"(?xi)
            ^(?:https?://)?github\.com/
            (?P<user>[^/]+)
            /(?P<repo>[^/]+)
            /(?:blob/(?P<hash>[a-f0-9]{40})/)?(?P<path>.+?)(?:@(?P<commit_or_branch>[^@]+))?$
        ").unwrap();

        if let Some(caps) = re.captures(url) {
            let user = caps.name("user").unwrap().as_str();
            let repo = caps.name("repo").unwrap().as_str();
            let path = caps.name("path").unwrap().as_str();
            
            // Determine the branch or commit hash
            let commit_or_branch = match (caps.name("hash"), caps.name("commit_or_branch")) {
                (Some(hash), _) => hash.as_str(), // Hash from blob format takes precedence
                (_, Some(commit_or_branch)) => commit_or_branch.as_str(),
                _ => "master", // Default branch if none specified
            };

            let raw_url = format!(
                "https://raw.githubusercontent.com/{}/{}/{}/{}",
                user, repo, commit_or_branch, path
            );

            println!("{raw_url}");

            Ok(RemoteFileSource{
                commit_or_branch: commit_or_branch.to_owned(),
                download_url: raw_url.to_owned(),
                repo: repo.to_owned(),
                original_url: url.to_owned(),
                user: user.to_owned(),
                source_origin: SourceOrigin::GitHub
            })
        } else {
            Err(ErrorBase::new_box("Invalid GitHub URL"))
        }
    }
}

enum SourceOrigin {
    GitHub,
    GitLab,
    Unknown,
}

impl SourceOrigin {
    fn source_matcher(url: &str) -> SourceOrigin {
        if url.contains("github.com") {
            SourceOrigin::GitHub
        } else if url.contains("gitlab.com") {
            SourceOrigin::GitLab
        } else {
            SourceOrigin::Unknown
        }
    }
}

// All cases needs to be addressed: 

// https://github.com/MadL1me/RhythmGE/blob/68f7379aa960a365d7eb61577d536307334a4e2e/src/index.scss
// http://github.com/MadL1me/RhythmGE/blob/68f7379aa960a365d7eb61577d536307334a4e2e/src/index.scss
// github.com/MadL1me/RhythmGE/src/index.scss
// github.com/MadL1me/RhythmGE/src/index.scss@master
// github.com/MadL1me/RhythmGE/src/index.scss@68f7379aa960a365d7eb61577d536307334a4e2e
// https://github.com/MadL1me/RhythmGE/src/index.scss@master
// http://github.com/MadL1me/RhythmGE/src/index.scss
// http://github.com/MadL1me/RhythmGE/src/index.scss@68f7379aa960a365d7eb61577d536307334a4e2e