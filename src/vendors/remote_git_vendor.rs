use crate::{ExternalDep, VendoringOptions};

use std::process::Command;
use std::io::{self, Write};
use std::path::Path;
use std::{fs, str};
use regex::Regex;
use crate::vendors::AnyError;
use thiserror::Error;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use crate::os;
use crate::os::{clean_dir, copy_dir_all};

#[derive(Debug, Clone)]
pub struct GitVendorSource {
    pub original_source: String,
    clone_url_ssh: String,
    clone_url_https: String,
    clone_dir: String,
    commit_or_branch: String,
    glob_path: String,
    download_folder: String,
}

#[derive(Debug, Error)]
pub enum VendorError {
    #[error("Failed to clean vendor directory")]
    FolderCleanError,
    #[error("Command execution failed: {0}")]
    CommandExecutionError(String),
    #[error("Error fetching glob path")]
    FetchGlobPathError(#[from] io::Error),
}

pub fn vendor(source: GitVendorSource, opts: VendoringOptions) -> Result<(), AnyError> {
    let bar = ProgressBar::new(10);
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:20.cyan/blue} {pos:>6}/{len:6} {msg}").unwrap());
    bar.enable_steady_tick(Duration::from_millis(50));

    let clone_dir = format!("{}/{}", opts.vendor_dir, source.clone_dir);
    let copy_path = format!("{}/{}", opts.vendor_dir, source.download_folder);

    bar.inc(1);

    fetch_glob_path(
        &source.clone_url_ssh,
        &source.commit_or_branch,
        &source.glob_path,
        &clone_dir,
        &bar)?;

    bar.inc(1);

    let root_dir: Vec<&str> = source.clone_dir.split('/').collect();

    os::set_permissions(Path::new("vendor"))?;

    let copy_to: String = copy_path
        .split("/")
        .collect::<Vec<_>>()  // Collect parts into a Vec<&str>
        .split_last().unwrap().1  // Get the slice of all but the last element
        .join("/");

    copy_dir_all(Path::new(clone_dir.as_str()), Path::new(copy_to.as_str()))?;

    fs::remove_dir_all(format!("{}/{}", opts.vendor_dir.clone(), root_dir[0]))?;

    bar.finish();

    Ok(())
}

fn run_command(cmd: &mut Command) -> Result<(), VendorError> {
    let output = cmd.output().map_err(|e| VendorError::CommandExecutionError(e.to_string()))?;
    if !output.status.success() {
        return Err(VendorError::CommandExecutionError(format!(
            "Command failed with status: {:?}\nstderr: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(())
}

fn fetch_glob_path(repo_url: &str, branch: &str, glob_path: &str, target_dir: &str, bar: &ProgressBar) -> Result<(), VendorError> {
    // Initialize a new empty repository in the target directory
    run_command(Command::new("git").arg("init").arg("--bare").arg(format!("{}/.git", target_dir)))?;

    bar.inc(1);
    bar.set_message("add git remote");

    // Add a remote pointing to the repository URL
    run_command(Command::new("git")
        .arg("--git-dir")
        .arg(format!("{}/.git", target_dir))
        .arg("--work-tree")
        .arg(target_dir)
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(repo_url))?;

    bar.inc(1);
    bar.set_message("setup sparse checkout");

    // Set up sparse checkout
    run_command(Command::new("git")
        .arg("--git-dir")
        .arg(format!("{}/.git", target_dir))
        .arg("--work-tree")
        .arg(target_dir)
        .arg("config")
        .arg("core.sparseCheckout")
        .arg("true"))?;

    bar.inc(1);

    // Write the file path to the sparse-checkout file
    {
        use std::fs::OpenOptions;

        let sparse_checkout_path = format!("{}/.git/info/sparse-checkout", target_dir);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(sparse_checkout_path)?;

        writeln!(file, "{}", glob_path)?;
        bar.inc(1);
    }

    bar.set_message("fetching branch");

    // Fetch the specified branch
    run_command(Command::new("git")
        .arg("--git-dir")
        .arg(format!("{}/.git", target_dir))
        .arg("--work-tree")
        .arg(target_dir)
        .arg("fetch")
        .arg("--progress")
        .arg("origin")
        .arg(branch))?;

    bar.inc(1);
    bar.set_message("checkout branch");

    // Checkout the specified branch
    run_command(Command::new("git")
        .arg("--git-dir")
        .arg(format!("{}/.git", target_dir))
        .arg("--work-tree")
        .arg(target_dir)
        .arg("checkout")
        .arg(branch))?;

    bar.inc(1);
    bar.finish_with_message("COMPLETED");

    Ok(())
}

#[derive(Debug, Error)]
pub enum GitVendorError {
    #[error("Invalid URL format")]
    InvalidUrlFormat,
    #[error("File field is present but glob_path is a glob pattern")]
    GlobPathError,
}

pub fn get_git_vendor_source(dep: ExternalDep, default_branch: &str) -> Result<GitVendorSource, AnyError> {
    let re = Regex::new(r"(?x)
        (?P<protocol>https?://)?                   # Optional protocol
        (?P<host>[^/]+)                            # Hostname
        /(?P<user_repo>[^/]+/[^/]+)                # User and repo
        (?:/blob/(?P<commit>[^/]+))?               # Optional commit hash
        (?:/(?P<path>[^@]+))?                      # File path
        (?:@(?P<branch_or_commit>[^/]+))?          # Optional branch or commit hash
    ").unwrap();

    let captures = re.captures(&dep.import).ok_or(GitVendorError::InvalidUrlFormat)?;

    let host = captures.name("host").map_or("", |m| m.as_str());
    let user_repo = captures.name("user_repo").map_or("", |m| m.as_str());
    let path = captures.name("path").map_or("", |m| m.as_str());
    let commit = captures.name("commit");
    let branch_or_commit = captures.name("branch_or_commit");

    let commit_or_branch = if let Some(commit) = commit {
        commit.as_str()
    } else if let Some(branch_or_commit) = branch_or_commit {
        branch_or_commit.as_str()
    } else if let Some(ref tag) = dep.tag {
        tag.as_str()
    } else {
        default_branch
    }.to_string();

    if dep.target_file.is_some() && path.contains('*') {
        return Err(Box::new(GitVendorError::GlobPathError));
    }

    let clone_url_ssh = format!("git@{}:{}.git", host, user_repo);
    let clone_url_https = format!("https://{}:{}.git", host, user_repo);
    let clone_dir = format!("clone@{}/{}/{}", host, user_repo, commit_or_branch);
    let download_dir = format!("{}/{}/{}", host, user_repo, commit_or_branch);

    Ok(GitVendorSource {
        original_source: dep.import.clone(),
        clone_url_ssh,
        clone_url_https,
        clone_dir,
        commit_or_branch,
        glob_path: path.to_string(),
        download_folder: download_dir
    })
}
