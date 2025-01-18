use crate::build::{build, get_build_options, get_target, Config};
use crate::cli::Build;
use crate::command;

pub fn run(args: &Build) -> Result<String, String> {
    build(args)?;

    let config = get_build_options(args)?;

    let command = if args.benchmark {
        format!("{}/benchmark", get_target(&config))
    } else {
        format!("{}/{}", get_target(&config), &config.package.name)
    };

    println!("Running {}", command);
    let mut process = match command::spawn(&command) {
        Ok(process) => process,
        Err(error) => {
            return Err(format!("Failed to run command: {}", error));
        }
    };

    match process.wait() {
        Ok(_) => {
            if args.benchmark {
                println!("--------------------------------------------------------");

                match command::spawn(&format!("gprof --brief {}/benchmark", get_target(&config))) {
                    Ok(mut process) => match process.wait() {
                        Ok(_) => Ok("".to_string()),
                        Err(e) => {
                            return Err(format!("Failed to wait for command: {}", e));
                        }
                    },
                    Err(error) => {
                        return Err(format!("Failed to run command: {}", error));
                    }
                }
            } else {
                Ok("".to_string())
            }
        }
        Err(e) => Err(format!("Failed to wait for command: {}", e)),
    }
}

pub fn get_memory_string(config: &Config) -> String {
    let mut memory_string = String::new();

    if !config.memory.leak_check.is_empty() {
        memory_string.push_str("--leak-check=");
        memory_string.push_str(&config.memory.leak_check);
    }

    if !config.memory.show_leak_kinds.is_empty() {
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

pub fn memory_run(args: &Build) -> Result<String, String> {
    build(args)?;

    let config = get_build_options(args)?;

    let memory_string = get_memory_string(&config);

    let command = format!(
        "valgrind {} ./{}/{}",
        memory_string,
        get_target(&config),
        if args.benchmark {
            "benchmark"
        } else {
            &config.package.name
        }
    );

    println!("Running {}", command);
    let mut process = match command::spawn(&command) {
        Ok(process) => process,
        Err(error) => {
            return Err(format!("Failed to run command: {}", error));
        }
    };

    match process.wait() {
        Ok(_) => {
            if args.benchmark {
                println!("--------------------------------------------------------");

                match command::spawn(&format!("gprof --brief {}/benchmark", get_target(&config))) {
                    Ok(mut process) => match process.wait() {
                        Ok(_) => Ok("".to_string()),
                        Err(e) => {
                            return Err(format!("Failed to wait for command: {}", e));
                        }
                    },
                    Err(error) => {
                        return Err(format!("Failed to run command: {}", error));
                    }
                }
            } else {
                Ok("".to_string())
            }
        }
        Err(e) => Err(format!("Failed to wait for command: {}", e)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::build::{Memory, Mode};

    #[test]
    fn test_get_memory_string() {
        let config = Config {
            package: Default::default(),
            mode: Some(Mode::Debug),
            benchmark: Some(false),
            debug: Default::default(),
            release: Default::default(),
            memory: Memory {
                leak_check: "full".to_string(),
                show_leak_kinds: "definite".to_string(),
                track_origins: true,
            },
        };

        let memory_string = get_memory_string(&config);
        assert_eq!(
            memory_string,
            "--leak-check=full --show-leak-kinds=definite --track-origins=yes"
        );
    }

    #[test]
    fn test_get_memory_string_empty() {
        let config = Config {
            package: Default::default(),
            mode: Some(Mode::Debug),
            benchmark: Some(false),
            debug: Default::default(),
            release: Default::default(),
            memory: Memory {
                leak_check: "".to_string(),
                show_leak_kinds: "".to_string(),
                track_origins: false,
            },
        };

        let memory_string = get_memory_string(&config);
        assert_eq!(memory_string, " --track-origins=no");
    }
}
