use std::fs::remove_dir;

pub fn clean() -> Result<Option<String>, String> {
    println!("Cleaning target directory");
    match remove_dir("c_target") {
        Ok(_) => Ok(Some("Cleaned target directory".to_string())),
        Err(e) => Err(format!("Failed to clean target directory: {}", e)),
    }
}
