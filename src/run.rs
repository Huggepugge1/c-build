use crate::build::{build, get_build_options};
use crate::cli::Build;
use crate::command;

pub fn run(args: &Build) -> Result<Option<String>, String> {
    match build(args) {
        Ok(Some(v)) => println!("{}", v),
        Ok(None) => (),
        Err(e) => return Err(e),
    }

    let build_options = match get_build_options() {
        Ok(config) => config,
        Err(e) => return Err(e),
    };

    let command = String::from("./")
        + match args.release {
            true => &build_options.release.path,
            false => &build_options.debug.path,
        }
        + &build_options.package.name;

    println!("Running {}", command);
    let mut process = match command::spawn(&command) {
        Ok(process) => process,
        Err(error) => {
            return Err(format!("Failed to run command: {}", error));
        }
    };

    match process.wait() {
        Ok(_) => Ok(None),
        Err(e) => Err(format!("Failed to wait for command: {}", e)),
    }
}

pub fn memory_run(args: &Build) -> Result<Option<String>, String> {
    match build(args) {
        Ok(Some(v)) => println!("{}", v),
        Ok(None) => (),
        Err(e) => return Err(e),
    }
    let build_options = match get_build_options() {
        Ok(config) => config,
        Err(e) => return Err(e),
    };

    let command = format!(
        "valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes ./{}/{}",
        match args.release {
            true => &build_options.release.path,
            false => &build_options.debug.path,
        },
        build_options.package.name
    );
    println!("Running {}", command);
    let mut process = match command::spawn(&command) {
        Ok(process) => process,
        Err(error) => {
            return Err(format!("Failed to run command: {}", error));
        }
    };

    match process.wait() {
        Ok(_) => Ok(None),
        Err(e) => Err(format!("Failed to wait for command: {}", e)),
    }
}
