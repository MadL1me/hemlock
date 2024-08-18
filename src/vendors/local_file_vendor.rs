use std::fs;
use std::path::{Path, PathBuf};
use crate::{ExternalDep, LocalDep, VendoringOptions};
use crate::os::copy_dir_all;
use crate::vendors::AnyError;
use crate::vendors::remote_git_vendor::{GitVendorSource, VendorError};

#[derive(Debug, Clone)]
pub struct LocalVendorSource {
    pub original_source: String,
    pub glob_path: String,
    pub download_folder: String,
}

pub fn get_local_vendor_source(dep: LocalDep, vendor_dir: &str) -> Result<LocalVendorSource, AnyError> {
    let (import, target_dir) = match dep {
        LocalDep::Simple(import) => (import, None),
        LocalDep::Structured(structured_dep) => (structured_dep.import, structured_dep.target_dir),
    };

    let import_path = Path::new(&import);

    let download_folder = match target_dir {
        None => {
            let vendor_base = Path::new(vendor_dir);
            if import_path.is_file() {
                vendor_base.join(import_path.parent().unwrap()).to_string_lossy().into_owned()
            } else {
                vendor_base.to_string_lossy().into_owned()
            }
        }
        Some(value) => String::from(vendor_dir) + "/" + &*value,
    };

    Ok(LocalVendorSource {
        original_source: import.to_string(),
        glob_path: import.to_string(),
        download_folder,
    })
}

pub fn local_vendor(source: LocalVendorSource, opts: VendoringOptions) -> Result<(), AnyError> {

    fs::create_dir_all(source.download_folder.clone())
        .map_err(|err| VendorError::CommandExecutionError(String::from(err.to_string())))?;

    copy_dir_all(Path::new(source.original_source.as_str()), Path::new(source.download_folder.as_str()))?;

    // fs::copy(source.original_source, source.download_folder)
    //     .map_err(|err| VendorError::CommandExecutionError(String::from(err.to_string())))?;

    Ok(())
}