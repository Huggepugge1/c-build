use super::build;
use super::test_framework;

use crate::build::get_build_options;
use crate::cli::Build;
use crate::command::spawn;
use crate::run::get_memory_string;

pub fn run() -> Result<Option<String>, String> {
    test_framework::write_tests_to_file();
    build::build()?;
    let command = "c_target/test/test";

    println!("Running tests...");
    match spawn(command) {
        Ok(mut process) => match process.wait() {
            Ok(_) => Ok(None),
            Err(e) => Err(format!("Failed to wait for command: {}", e)),
        },
        Err(e) => Err(format!("Failed to run tests: {}", e)),
    }
}

pub fn memory_run() -> Result<Option<String>, String> {
    test_framework::write_tests_to_file();
    build::build()?;
    let config = get_build_options(&Build { release: false })?;
    let command = format!("valgrind {} c_target/test/test", get_memory_string(&config),);

    println!("Running tests with memory check...");
    match spawn(&command) {
        Ok(mut process) => match process.wait() {
            Ok(_) => Ok(None),
            Err(e) => Err(format!("Failed to wait for command: {}", e)),
        },
        Err(e) => Err(format!("Failed to run tests: {}", e)),
    }
}
