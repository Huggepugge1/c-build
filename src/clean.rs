use std::fs::remove_dir_all;

pub fn clean() -> Result<Option<String>, String> {
    match remove_dir_all("c_target") {
        Ok(_) => Ok(Some("Cleaned target directory".to_string())),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => Ok(Some("No directory to clean".to_string())),
            _ => Err(format!("Failed to clean target directory: {}", e)),
        },
    }
}
