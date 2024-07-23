pub enum SourceOrigin {
    GitHubUrl,
    GitHubGlob,
    GitLabUrl,
    GitLabGlob,
    LocalGlob,
    Unknown,
}

impl SourceOrigin {
    pub fn match_by_path(path: &str) -> SourceOrigin {
        // Regex patterns to identify URL and glob patterns
        let github_url_pattern = regex::Regex::new(r"^(https?://)?(www\.)?github\.com/[^/]+/[^/]+(/.*)?$").unwrap();
        let github_glob_pattern = regex::Regex::new(r"^(https?://)?(www\.)?github\.com/[^/]+/[^/]+/.*\*$").unwrap();
        let gitlab_url_pattern = regex::Regex::new(r"^(https?://)?(www\.)?gitlab\.com/[^/]+/[^/]+(/.*)?$").unwrap();
        let gitlab_glob_pattern = regex::Regex::new(r"^(https?://)?(www\.)?gitlab\.com/[^/]+/[^/]+/.*\*$").unwrap();

        if github_url_pattern.is_match(path) {
            SourceOrigin::GitHubUrl
        } else if github_glob_pattern.is_match(path) {
            SourceOrigin::GitHubGlob
        } else if gitlab_url_pattern.is_match(path) {
            SourceOrigin::GitLabUrl
        } else if gitlab_glob_pattern.is_match(path) {
            SourceOrigin::GitLabGlob
        } else if path.contains('*') {
            // Assuming any other pattern with '*' is a local glob
            SourceOrigin::LocalGlob
        } else {
            SourceOrigin::Unknown
        }
    }
}

pub struct LocalFilesSource {
    pub regexp_path: String
}

pub struct RemoteGitFileSource {
    pub original_url: String,
    pub source_origin: SourceOrigin,

    pub user: String,
    pub repo: String,
    pub commit_or_branch: String,
    pub download_url: String,
}