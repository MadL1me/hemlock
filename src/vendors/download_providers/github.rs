// use regex::Regex;
// use crate::{AnyError, VendorSource};
// use crate::errors::ErrorBase;
// use crate::remote_sources::{RemoteGitFileSource, SourceOrigin};
// use crate::vendors::download_providers::DownloadUrlProvider;
//
// pub struct GithubUrlProvider {}
//
// impl DownloadUrlProvider for GithubUrlProvider {
//     fn get_download_url(&self, source: VendorSource) -> Result<RemoteGitFileSource, AnyError> {
//         let url = source.source.as_str();
//
//         let re = Regex::new(r"(?xi)
//             ^(?:https?://)?github\.com/
//             (?P<user>[^/]+)
//             /(?P<repo>[^/]+)
//             /(?:blob/(?P<hash>[a-f0-9]{40})/)?(?P<path>.+?)(?:@(?P<commit_or_branch>[^@]+))?$
//         ").unwrap();
//
//         if let Some(caps) = re.captures(url) {
//             let user = caps.name("user").unwrap().as_str();
//             let repo = caps.name("repo").unwrap().as_str();
//             let path = caps.name("path").unwrap().as_str();
//
//             // Determine the branch or commit hash
//             let commit_or_branch = match (caps.name("hash"), caps.name("commit_or_branch")) {
//                 (Some(hash), _) => hash.as_str(), // Hash from blob format takes precedence
//                 (_, Some(commit_or_branch)) => commit_or_branch.as_str(),
//                 _ => "master", // Default branch if none specified
//             };
//
//             let raw_url = format!(
//                 "https://raw.githubusercontent.com/{}/{}/{}/{}",
//                 user, repo, commit_or_branch, path
//             );
//
//             println!("{raw_url}");
//
//             Ok(RemoteGitFileSource {
//                 commit_or_branch: commit_or_branch.to_owned(),
//                 download_url: raw_url.to_owned(),
//                 repo: repo.to_owned(),
//                 original_url: url.to_owned(),
//                 user: user.to_owned(),
//                 source_origin: SourceOrigin::GitHubUrl
//             })
//         } else {
//             Err(ErrorBase::new_box("Invalid GitHub URL"))
//         }
//     }
// }
