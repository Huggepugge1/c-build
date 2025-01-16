use crate::build::{build, get_build_options, get_target, Config};
use crate::cli::Build;
use crate::command;

pub fn run(args: &Build) -> Result<Option<String>, String> {
    match build(args) {
        Ok(Some(v)) => println!("{}", v),
        Ok(None) => (),
        Err(e) => return Err(e),
    }

    let config = get_build_options(args)?;

    let command = format!("./{}/{}", get_target(&config.mode), config.package.name);

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

fn get_memory_string(config: &Config) -> String {
    let mut memory_string = String::new();

    if config.memory.leak_check != "" {
        memory_string.push_str("--leak-check=");
        memory_string.push_str(&config.memory.leak_check);
    }

    if config.memory.show_leak_kinds != "" {
        memory_string.push_str(" --show-leak-kinds=");
        memory_string.push_str(&config.memory.show_leak_kinds);
    }

    if config.memory.track_origins {
        memory_string.push_str(" --track-origins=yes");
    } else {
        memory_string.push_str(" --track-origins=no");
    }

    memory_string
}

pub fn memory_run(args: &Build) -> Result<Option<String>, String> {
    match build(args) {
        Ok(Some(v)) => println!("{}", v),
        Ok(None) => (),
        Err(e) => return Err(e),
    }

    let config = match get_build_options(args) {
        Ok(config) => config,
        Err(e) => return Err(e),
    };

    let memory_string = get_memory_string(&config);

    let command = format!(
        "valgrind {} ./{}/{}",
        memory_string,
        get_target(&config.mode),
        config.package.name
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
