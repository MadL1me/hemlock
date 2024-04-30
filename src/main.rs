use std::{fs::{self, File}, io, os::macos::raw, path::PathBuf};
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
    vendor_dir: String,
    local_deps: Vec<String>,
    external_deps: Vec<String>,
}

type ErrTrait = Box<dyn std::error::Error>;

fn main() -> Result<(), ErrTrait> {
    let args = CliArgs::parse();
    
    // Check if the config path was provided
    let config_path = args.config.ok_or("No config path provided")?;

    // Read the configuration file
    let file_res = fs::read_to_string(config_path)
        .map_err(|err| format!("Failed to read the config file: {}", err))?;

    // Parse the YAML content
    let config: YamlConfigV1 = serde_yaml::from_str(&file_res)
        .map_err(|err| format!("Failed to parse config: {}", err))?;

    // Use the parsed config (here, just printing it)
    if args.verbose {
        println!("Verbose mode enabled");
    }

    println!("Config loaded successfully: {:?}", config);

    for remote_dep in config.external_deps {
        download_locally(&remote_dep[..], &config.vendor_dir[..], args.verbose)?;
    }

    Ok(())
}

fn download_locally(url: &str, dir: &str, verbose: bool) -> Result<&'static str, ErrTrait> {
    let download_src: Result<String, &'static str>;

    match source_matcher(&url) {
        SourceOrigin::GitHub => download_src = github_to_raw(&url),
        SourceOrigin::GitLab => todo!(),
        SourceOrigin::Unknown => todo!(),
    }

    let raw_url = download_src
        .map_err(|err| format!("Failed to read the config file: {err}"))?;

    let response = Client::new().get(&raw_url).send()?;

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

    Ok("123")
}

enum SourceOrigin {
    GitHub,
    GitLab,
    Unknown,
}

fn source_matcher(url: &str) -> SourceOrigin {
    if url.contains("github.com") {
        SourceOrigin::GitHub
    } else if url.contains("gitlab.com") {
        SourceOrigin::GitLab
    } else {
        SourceOrigin::Unknown
    }
}

fn github_to_raw(url: &str) -> Result<String, &'static str> {
    let re = Regex::new(r"(?x)
        ^(?:https?://)?github\.com/
        (?P<user>[^/]+)
        /(?P<repo>[^/]+)
        /(?P<path>blob/)?(?P<branch>[^/]+)?
        /(?P<file_path>.+?)(?:@(?P<commit>[a-f0-9]{40}))?$
    ").unwrap();

    if let Some(caps) = re.captures(url) {
        let user = caps.name("user").unwrap().as_str();
        let repo = caps.name("repo").unwrap().as_str();
        let file_path = caps.name("file_path").unwrap().as_str();
        let branch = caps.name("branch").map_or("master", |m| m.as_str());
        let commit = caps.name("commit").map_or(branch, |m| m.as_str());

        let raw_url = format!(
            "https://raw.githubusercontent.com/{}/{}/{}/{}",
            user, repo, commit, file_path
        );

        println!("{raw_url}");

        Ok(raw_url)
    } else {
        Err("Invalid GitHub URL")
    }
}

// https://github.com/MadL1me/RhythmGE/blob/68f7379aa960a365d7eb61577d536307334a4e2e/src/index.scss
// http://github.com/MadL1me/RhythmGE/blob/68f7379aa960a365d7eb61577d536307334a4e2e/src/index.scss
// github.com/MadL1me/RhythmGE/src/index.scss
// github.com/MadL1me/RhythmGE/src/index.scss@master
// github.com/MadL1me/RhythmGE/src/index.scss@68f7379aa960a365d7eb61577d536307334a4e2e
// https://github.com/MadL1me/RhythmGE/src/index.scss@master
// http://github.com/MadL1me/RhythmGE/src/index.scss
// http://github.com/MadL1me/RhythmGE/src/index.scss@68f7379aa960a365d7eb61577d536307334a4e2e