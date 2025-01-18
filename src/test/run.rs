use super::build;

use crate::build::{get_build_options, get_target};
use crate::cli::{Build, Test};
use crate::command::spawn;
use crate::run::get_memory_string;

pub fn run(test: &Test) -> Result<String, String> {
    let build = Build {
        release: test.release,
        benchmark: false,
    };
    build::build(&build)?;

    let config = get_build_options(&build)?;
    let command = format!("{}/test", get_target(&config));

    println!("Running tests...");
    match spawn(&command) {
        Ok(mut process) => match process.wait() {
            Ok(_) => Ok(String::new()),
            Err(e) => Err(format!("Failed to wait for command: {}", e)),
        },
        Err(e) => Err(format!("Failed to run tests: {}", e)),
    }
}

pub fn memory_run(test: &Test) -> Result<String, String> {
    let build = Build {
        release: test.release,
        benchmark: false,
    };
    build::build(&build)?;

    let config = get_build_options(&build)?;
    let command = format!(
        "valgrind {} {}/test",
        get_memory_string(&config),
        get_target(&config),
    );

    println!("Running tests with memory check...");
    match spawn(&command) {
        Ok(mut process) => match process.wait() {
            Ok(_) => Ok(String::new()),
            Err(e) => Err(format!("Failed to wait for command: {}", e)),
        },
        Err(e) => Err(format!("Failed to run tests: {}", e)),
    }
}
