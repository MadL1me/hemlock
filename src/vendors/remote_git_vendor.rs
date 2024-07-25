use crate::{ExternalDep, VendoringOptions};

use std::process::Command;
use std::io::{self, Write};
use std::path::Path;
use std::{fs, str};
use regex::Regex;
use crate::vendors::AnyError;
use thiserror::Error;
use std::os::unix::fs::PermissionsExt;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, Clone)]
pub struct GitVendorSource {
    pub original_source: String,
    clone_path: String,
    clone_dir: String,
    commit_or_branch: String,
    glob_path: String,
    download_folder: String,
}

pub fn vendor(source: GitVendorSource, opts: VendoringOptions) -> Result<(), AnyError> {
    let bar = ProgressBar::new(10);
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap());
    bar.enable_steady_tick(Duration::from_millis(50));

    let clone_dir = format!("{}/{}", opts.vendor_dir, source.clone_dir);
    let copy_path = format!("{}/{}", opts.vendor_dir, source.download_folder);

    bar.inc(1);

    fetch_glob_path(
        &*source.clone_path,
        &*source.commit_or_branch,
        &*source.glob_path,
        &*clone_dir,
        &bar).expect("ADADA");

    bar.inc(1);

    let root_dir: Vec<&str> = source.clone_dir.split('/').collect();

    set_permissions(Path::new("vendor")).expect("123");

    copy_dir_all(Path::new(clone_dir.as_str()), Path::new(copy_path.as_str())).expect("TODO: panic message");

    fs::remove_dir_all(format!("{}/{}", opts.vendor_dir.clone(), root_dir[0])).expect("FUUUCK");

    bar.finish();

    Ok(())
}

fn run_command(cmd: &mut Command) -> io::Result<()> {
    let output = cmd.output()?;
    if !output.status.success() {
        eprintln!("Command failed with status: {:?}", output.status);
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        Err(io::Error::new(io::ErrorKind::Other, "Command execution failed"))
    } else {
        let s = String::from_utf8(output.stdout).expect("NOT VALID UTF8");
        // println!("{:?}", s);
        Ok(())
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn fetch_glob_path(repo_url: &str, branch: &str, glob_path: &str, target_dir: &str, bar: &ProgressBar) -> io::Result<()> {
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

fn set_permissions(path: &std::path::Path) -> io::Result<()> {
    if path.is_file() {
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o644); // rw-r--r--
        fs::set_permissions(&path, perms)?;
    } else if path.is_dir() {
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o755); // rwxr-xr-x
        fs::set_permissions(&path, perms)?;

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            set_permissions(&entry_path)?; // Recurse into directories
        }
    }
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

    let clone_url = format!("git@{}:{}.git", host, user_repo);
    let clone_dir = format!("clone@{}/{}/{}", host, user_repo, commit_or_branch);
    let download_dir = format!("{}/{}/{}", host, user_repo, commit_or_branch);

    Ok(GitVendorSource {
        original_source: dep.import.clone(),
        clone_path: clone_url,
        clone_dir,
        commit_or_branch,
        glob_path: path.to_string(),
        download_folder: download_dir
    })
}
