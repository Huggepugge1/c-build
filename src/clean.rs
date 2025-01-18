use std::fs::remove_dir_all;

pub fn clean() -> Result<String, String> {
    match remove_dir_all("c_target") {
        Ok(_) => Ok("Cleaned target directory".to_string()),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => Ok("No directory to clean".to_string()),
            _ => Err(format!("Failed to clean target directory: {}", e)),
        },
    }
}
