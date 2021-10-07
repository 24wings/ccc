use std::path::PathBuf;

/// get absolute_path from current_dir
pub fn get_absolute_path_from_current_dir(path_string: String) -> std::io::Result<PathBuf> {
    let mut full_path: PathBuf;
    let path = std::path::Path::new(&path_string);
    if path.is_relative() {
        let current_dir = std::env::current_dir()?;
        full_path = current_dir.to_path_buf();
        full_path.push(path.to_path_buf());
    } else {
        full_path = path.to_path_buf();
    }
    Ok(full_path)
}
