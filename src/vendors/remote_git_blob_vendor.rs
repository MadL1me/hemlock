use crate::vendors::download_providers::DownloadUrlProvider;
use crate::vendors::FileVendor;
use crate::VendorSource;

pub struct RemoteGitBlobVendor {
    url_provider: Box<dyn DownloadUrlProvider>
}

impl FileVendor for RemoteGitBlobVendor {
    fn vendor(&self, source: VendorSource) -> Result<&str, crate::AnyError> {
        todo!()
    }
}

pub struct LocalVendor {

}
