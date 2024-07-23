pub mod github;
pub mod gitlab;

use crate::{AnyError, VendorSource};
use crate::remote_sources::{RemoteGitFileSource, SourceOrigin};

fn get_url_provider(path: &str) -> Box<dyn DownloadUrlProvider> {
    let source_type = SourceOrigin::match_by_path(path);

    match source_type {
        SourceOrigin::GitHubUrl => {
            panic!("123")
        }
        SourceOrigin::GitLabUrl => {
            panic!("123")
        }
        _ => {
            panic!("123")
        }
    }
}

pub trait DownloadUrlProvider {
    fn get_download_url(&self, source: VendorSource) -> Result<RemoteGitFileSource, AnyError>;
}