use std::fs;
use std::io;
use std::io::{Error, ErrorKind};
use std::path::Path;
use glob::glob;
use crate::vendors::AnyError;
use crate::vendors::remote_git_vendor::VendorError;

pub fn set_permissions(path: &Path) -> io::Result<()> {
    if path.is_file() {
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_readonly(false);
        fs::set_permissions(&path, perms)?;
    } else if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            set_permissions(&entry_path)?; // Recurse into directories
        }
    }
    Ok(())
}

pub fn clean_dir(path: &str) -> io::Result<()>  {
    fs::remove_dir_all(path)
}

// pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
//     fs::create_dir_all(&dst)?;
//     for entry in fs::read_dir(src)? {
//         let entry = entry?;
//         let ty = entry.file_type()?;
//         if ty.is_dir() {
//             copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
//         } else {
//             fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
//         }
//     }
//     Ok(())
// }

// pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), AnyError> {
//     let src_path = src.as_ref();
//     let dst_path = dst.as_ref();
//
//     // Prevent copying into itself or a subdirectory of itself
//     if dst_path.starts_with(src_path) {
//         return Err(Box::new(std::io::Error::new(
//             std::io::ErrorKind::InvalidInput,
//             "Destination path is within the source path",
//         )));
//     }
//
//     let pattern = src_path.to_str().ok_or("Invalid source path")?;
//     let src_base = src_path.parent().unwrap_or_else(|| Path::new(""));
//
//     for entry in glob(pattern)? {
//         match entry {
//             Ok(path) => {
//                 let relative_path = match path.strip_prefix(src_base) {
//                     Ok(p) => p,
//                     Err(_) => path.as_path(),
//                 };
//                 let dest_path = dst_path.join(relative_path);
//
//                 if path.is_dir() {
//                     fs::create_dir_all(&dest_path)?;
//                     copy_dir_all(&path, &dest_path)?;
//                 } else {
//                     if let Some(parent) = dest_path.parent() {
//                         fs::create_dir_all(parent)?;
//                     }
//                     fs::copy(&path, &dest_path)?;
//                 }
//             }
//             Err(e) => println!("Error while processing glob pattern: {:?}", e),
//         }
//     }
//     Ok(())
// }

// pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), AnyError> {
//     let src_path = src.as_ref();
//     let dst_path = dst.as_ref();
//
//     if src_path.is_dir() {
//         fs::create_dir_all(&dst_path)?;
//         for entry in fs::read_dir(src_path)? {
//             let entry = entry?;
//             let ty = entry.file_type()?;
//             let entry_path = entry.path();
//             let dest_path = dst_path.join(entry.file_name());
//             if ty.is_dir() {
//                 copy_dir_all(&entry_path, &dest_path)?;
//             } else {
//                 fs::copy(&entry_path, &dest_path)?;
//             }
//         }
//     } else {
//         // Handle blobs and default file names
//         let pattern = src_path.to_str().ok_or("Invalid source path")?;
//         for entry in glob(pattern)? {
//             match entry {
//                 Ok(path) => {
//                     let relative_path = path.strip_prefix(src_path.parent().unwrap_or(src_path)).unwrap_or(&path);
//                     let dest_path = dst_path.join(relative_path);
//                     if path.is_dir() {
//                         fs::create_dir_all(&dest_path)?;
//                     } else {
//                         if let Some(parent) = dest_path.parent() {
//                             fs::create_dir_all(parent)?;
//                         }
//                         fs::copy(&path, &dest_path)?;
//                     }
//                 }
//                 Err(e) => println!("Error while processing glob pattern: {:?}", e),
//             }
//         }
//     }
//     Ok(())
// }

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), AnyError> {
    let src_path = src.as_ref();
    let dst_path = dst.as_ref();

    if src_path.is_dir() {
        let root = dst_path.join(src_path.file_name().unwrap());
        fs::create_dir_all(&root)?;
        for entry in fs::read_dir(src_path)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            let entry_path = entry.path();
            let dest_path = root.join(entry.file_name());
            if ty.is_dir() {
                copy_dir_all(&entry_path, &root)?;
            } else {
                fs::copy(&entry_path, &dest_path)?;
            }
        }
    } else {
        // Handle blobs and default file names
        let pattern = src_path.to_str().ok_or("Invalid source path")?;
        let src_base = src_path.parent().unwrap_or_else(|| Path::new(""));
        for entry in glob(pattern)? {
            match entry {
                Ok(path) => {
                    let relative_path = match path.strip_prefix(src_base) {
                        Ok(p) => p,
                        Err(_) => path.as_path(),
                    };
                    let dest_path = dst_path.join(relative_path);
                    if path.is_dir() {
                        fs::create_dir_all(&dest_path)?;
                    } else {
                        if let Some(parent) = dest_path.parent() {
                            fs::create_dir_all(parent)?;
                        }
                        fs::copy(&path, &dest_path)?;
                    }
                }
                Err(e) => println!("Error while processing glob pattern: {:?}", e),
            }
        }
    }
    Ok(())
}