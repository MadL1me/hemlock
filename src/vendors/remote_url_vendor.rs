use std::{fs, io};
use std::fs::File;
use std::path::Path;
use reqwest::blocking::Client;
use crate::vendors::download_providers::DownloadUrlProvider;
use crate::vendors::FileVendor;
use crate::VendorSource;

pub(crate) struct RemoteUrlVendor {
    pub(crate) url_provider: Box<dyn DownloadUrlProvider>
}

impl FileVendor for RemoteUrlVendor {
    fn vendor(&self, source: VendorSource) -> Result<&str, crate::AnyError> {
        let path = &source.source;
        let source = self.url_provider.get_download_url(source.clone())?;

        let response = Client::new().get(source.download_url).send()?;

        let filename = path.split('/').last().ok_or("Error")?;
        let dir = "vendor";

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
}
