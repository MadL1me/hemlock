// use regex::Regex;
// use crate::{AnyError, VendorSource};
// use crate::errors::ErrorBase;
// use crate::remote_sources::{RemoteGitFileSource, SourceOrigin};
// use crate::vendors::download_providers::DownloadUrlProvider;
//
// pub struct GitlabUrlProvider {}
//
// impl DownloadUrlProvider for GitlabUrlProvider {
//     fn get_download_url(&self, source: VendorSource) -> Result<RemoteGitFileSource, AnyError> {
//         let url = source.source.as_str();
//
//         let re = Regex::new(r"(?xi)
//             ^(?:https?://)?gitlab\.com/
//             (?P<user>[^/]+)
//             /(?P<repo>[^/]+)
//             /(?:-/blob/(?P<commit_or_branch>[^/]+)/)?(?P<path>.+)$
//         ").unwrap();
//
//         if let Some(caps) = re.captures(url) {
//             let user = caps.name("user").unwrap().as_str();
//             let repo = caps.name("repo").unwrap().as_str();
//             let path = caps.name("path").unwrap().as_str();
//
//             // Determine the branch or commit hash
//             let commit_or_branch = match caps.name("commit_or_branch") {
//                 Some(commit_or_branch) => commit_or_branch.as_str(),
//                 _ => "master", // Default branch if none specified
//             };
//
//             let raw_url = format!(
//                 "https://gitlab.com/{}/{}/-/raw/{}/{}",
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
//                 source_origin: SourceOrigin::GitLabUrl
//             })
//         } else {
//             Err(ErrorBase::new_box("Invalid GitLab URL"))
//         }
//     }
// }
