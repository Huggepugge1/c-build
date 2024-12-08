use crate::command;

pub fn clean() -> Result<Option<String>, String> {
    match command::output("rm -rf target") {
        Ok(_) => Ok(Some("Cleaned target directory".to_string())),
        Err(e) => Err(format!("Failed to clean target directory: {}", e)),
    }
}
