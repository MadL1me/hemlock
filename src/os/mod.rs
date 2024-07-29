use std::fs;
use std::io;
use std::path::Path;

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
