use crate::cli::Build;
use crate::command;
use crate::includes::{get_includes, Include, IncludeType};

use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Mode {
    Debug,
    Release,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mode: Option<Mode>,
    pub benchmark: Option<bool>,
    pub package: Package,
    pub debug: BuildArgs,
    pub release: BuildArgs,
    pub memory: Memory,
}

#[derive(Debug, Deserialize, Default)]
pub struct Package {
    pub name: String,
    #[allow(dead_code)]
    version: String,
    #[allow(dead_code)]
    authors: Vec<String>,
    src: String,
    benchmark: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct BuildArgs {
    debug: bool,
    optimization: u8,
    warnings: bool,
    pedantic: bool,
    std: String,
}

#[derive(Debug, Deserialize)]
pub struct Memory {
    pub leak_check: String,
    pub show_leak_kinds: String,
    pub track_origins: bool,
}

pub fn get_build_options(build: &Build) -> Result<Config, String> {
    let toml = match fs::read_to_string("c-build.toml") {
        Ok(toml) => toml,
        Err(e) => return Err(format!("Failed to read toml: {}", e)),
    };
    let mut config: Config = match toml::from_str(&toml) {
        Ok(config) => config,
        Err(e) => return Err(format!("Failed to parse toml: {}", e)),
    };

    config.mode = Some(match build.release {
        true => Mode::Release,
        false => Mode::Debug,
    });

    config.benchmark = Some(build.benchmark);

    Ok(config)
}

fn get_cflags(config: &Config) -> String {
    let mut cflags = String::new();
    let build = match config.mode.unwrap() {
        Mode::Debug => &config.debug,
        Mode::Release => &config.release,
    };
    cflags.push_str(&format!("-O{} ", build.optimization));
    if build.debug {
        cflags.push_str("-g ");
    }
    if build.warnings {
        cflags.push_str("-Wall ");
    }
    if build.pedantic {
        cflags.push_str("-pedantic ");
    }
    cflags.push_str(&format!("-std={} ", build.std));

    if config.benchmark.unwrap() {
        cflags.push_str("-pg ");
    }

    cflags
}

pub fn get_target(config: &Config) -> String {
    match config.benchmark.unwrap() {
        true => "c_target/benchmark",
        false => match config.mode.unwrap() {
            Mode::Debug => "c_target/debug",
            Mode::Release => "c_target/release",
        },
    }
    .to_string()
}

fn get_object_name(include: &Include) -> String {
    match &include.kind {
        IncludeType::Local(path) => {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| {
                    let mut hasher = DefaultHasher::new();
                    name.hash(&mut hasher);
                    hasher.finish().to_string()
                })
                .unwrap()
                + ".o"
        }
        IncludeType::System => unreachable!(),
    }
}

pub fn generate_build_command(includes: &Vec<Include>, config: &Config, main_file: &str) -> String {
    let mut command = format!("gcc {} {} ", &get_cflags(config), main_file);
    for include in includes {
        match &include.kind {
            IncludeType::Local(_) => {
                command.push_str(&format!(
                    "{}/obj/{} ",
                    get_target(config),
                    get_object_name(include)
                ));
            }
            IncludeType::System => (),
        }
    }

    command.push_str(&format!(
        "-o {}/{} ",
        get_target(config),
        if main_file.ends_with("tests.c") {
            "test"
        } else if config.benchmark.unwrap() {
            "benchmark"
        } else {
            &config.package.name
        }
    ));

    command
}

pub fn create_output_directory(config: &Config) -> Result<(), String> {
    let path = std::path::PathBuf::from(get_target(config).clone());
    if !path.exists() {
        match fs::create_dir_all(path) {
            Ok(_) => (),
            Err(e) => return Err(format!("Failed to create output directory: {}", e)),
        }
    }
    let obj_path = std::path::PathBuf::from(format!("{}/obj", get_target(config)));
    if !obj_path.exists() {
        match fs::create_dir_all(obj_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to create output directory: {}", e)),
        }
    } else {
        Ok(())
    }
}

fn should_build(include: &mut Include, config: &Config) -> Result<bool, String> {
    match &include.kind {
        IncludeType::System => return Ok(false),
        IncludeType::Local(path) => {
            let path = std::path::PathBuf::from(&config.package.src).join(path);
            let metadata = match fs::metadata(match path.with_extension("c").canonicalize() {
                Ok(path) => path,
                Err(_) => return Ok(true),
            }) {
                Ok(metadata) => metadata,
                Err(e) => return Err(format!("Failed to fetch metadata for file: {}", e)),
            };
            if let Ok(created_time) = metadata.modified() {
                match fs::metadata(
                    PathBuf::from(get_target(config))
                        .join("obj")
                        .join(get_object_name(include)),
                ) {
                    Ok(metadata) => {
                        if let Ok(obj_created_time) = metadata.modified() {
                            return Ok(created_time > obj_created_time);
                        }
                    }
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::NotFound => return Ok(true),
                        _ => return Err(format!("Failed to fetch metadata for file: {}", e)),
                    },
                };
            }
        }
    }
    Ok(true)
}

fn build_object(include: &mut Include, config: &Config) -> Result<(), String> {
    if !should_build(include, config)? {
        return Ok(());
    }

    let command = match &include.kind {
        IncludeType::Local(path) => format!(
            "gcc -fdiagnostics-color=always {} -c {} -o {}/obj/{}",
            get_cflags(config),
            path.with_extension("c").to_str().unwrap(),
            get_target(config),
            get_object_name(include),
        ),
        IncludeType::System => "".to_string(),
    };

    match command::output(&command) {
        Ok(status) => {
            if !status.success() {
                Err("Failed to build object file: {}".to_string())
            } else {
                Ok(())
            }
        }
        Err(e) => Err(format!("Failed to run command: {}", e)),
    }
}

pub fn build_object_files(includes: &Vec<Include>, config: &Config) -> Result<(), String> {
    includes
        .into_par_iter()
        .try_for_each(|include| match &include.kind {
            IncludeType::Local(_path) => match build_object(&mut include.clone(), config) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to build object files: {}", e)),
            },
            IncludeType::System => Ok(()),
        })
}

pub fn build(build: &Build) -> Result<String, String> {
    let config: Config = match get_build_options(build) {
        Ok(config) => config,
        Err(e) => return Err(e),
    };

    let path = if build.benchmark {
        std::path::PathBuf::from(&config.package.benchmark)
    } else {
        std::path::PathBuf::from(&config.package.src)
    };
    let includes = get_includes(path)?;

    create_output_directory(&config)?;
    build_object_files(&includes, &config)?;

    let main_file = format!(
        "{}/main.c",
        if build.benchmark {
            &config.package.benchmark
        } else {
            &config.package.src
        }
    );

    let build_command = generate_build_command(&includes, &config, &main_file);

    println!("Building {}/{}", get_target(&config), config.package.name);

    println!("Running command: {}", build_command);
    match command::output(&build_command) {
        Ok(status) => {
            if !status.success() {
                Err("Build not successful".to_string())
            } else {
                Ok("Build successful".to_string())
            }
        }
        Err(e) => Err(format!("Failed to run command: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_build_options() {
        let build = Build {
            release: false,
            benchmark: false,
        };
        let config = get_build_options(&build);
        assert_eq!(config.is_ok(), false);
    }

    #[test]
    fn test_get_cflags() {
        let config = Config {
            mode: Some(Mode::Debug),
            benchmark: Some(false),
            package: Package {
                name: "test".to_string(),
                src: "src".to_string(),
                ..Default::default()
            },
            debug: BuildArgs {
                debug: true,
                optimization: 0,
                warnings: false,
                pedantic: false,
                std: "c11".to_string(),
            },
            release: BuildArgs {
                debug: false,
                optimization: 0,
                warnings: false,
                pedantic: false,
                std: "c11".to_string(),
            },
            memory: Memory {
                leak_check: "".to_string(),
                show_leak_kinds: "".to_string(),
                track_origins: false,
            },
        };
        assert_eq!(get_cflags(&config), "-O0 -g -std=c11 ");
    }

    #[test]
    fn test_get_cflags_all() {
        let mut config = Config {
            mode: Some(Mode::Debug),
            benchmark: Some(false),
            package: Package {
                name: "test".to_string(),
                src: "src".to_string(),
                ..Default::default()
            },
            debug: BuildArgs {
                debug: true,
                optimization: 0,
                warnings: true,
                pedantic: true,
                std: "c11".to_string(),
            },
            release: BuildArgs {
                debug: false,
                optimization: 3,
                warnings: true,
                pedantic: true,
                std: "c11".to_string(),
            },
            memory: Memory {
                leak_check: "".to_string(),
                show_leak_kinds: "".to_string(),
                track_origins: false,
            },
        };
        assert_eq!(get_cflags(&config), "-O0 -g -Wall -pedantic -std=c11 ");
        config.mode = Some(Mode::Release);
        assert_eq!(get_cflags(&config), "-O3 -Wall -pedantic -std=c11 ");
    }

    #[test]
    fn test_get_target() {
        let mut config = Config {
            mode: Some(Mode::Debug),
            benchmark: Some(false),
            package: Package {
                name: "test".to_string(),
                src: "src".to_string(),
                ..Default::default()
            },
            debug: BuildArgs {
                debug: true,
                optimization: 0,
                warnings: false,
                pedantic: false,
                std: "c11".to_string(),
            },
            release: BuildArgs {
                debug: false,
                optimization: 0,
                warnings: false,
                pedantic: false,
                std: "c11".to_string(),
            },
            memory: Memory {
                leak_check: "".to_string(),
                show_leak_kinds: "".to_string(),
                track_origins: false,
            },
        };

        assert_eq!(get_target(&config), "c_target/debug");
        config.mode = Some(Mode::Release);
        assert_eq!(get_target(&config), "c_target/release");

        config.mode = Some(Mode::Debug);
        config.benchmark = Some(true);

        assert_eq!(get_target(&config), "c_target/benchmark");
        config.mode = Some(Mode::Release);
        assert_eq!(get_target(&config), "c_target/benchmark");
    }

    #[test]
    fn test_get_object_name() {
        let include = Include {
            kind: IncludeType::Local(PathBuf::from("test.c")),
        };
        assert_eq!(get_object_name(&include), "5868638564572808266.o");
    }

    #[test]
    fn test_generate_build_command() {
        let includes = vec![
            Include {
                kind: IncludeType::Local(PathBuf::from("test.c")),
            },
            Include {
                kind: IncludeType::Local(PathBuf::from("test2.c")),
            },
        ];
        let config = Config {
            mode: Some(Mode::Debug),
            benchmark: Some(false),
            package: Package {
                name: "test".to_string(),
                src: "src".to_string(),
                ..Default::default()
            },
            debug: BuildArgs {
                debug: true,
                optimization: 0,
                warnings: false,
                pedantic: false,
                std: "c11".to_string(),
            },
            release: BuildArgs {
                debug: false,
                optimization: 0,
                warnings: false,
                pedantic: false,
                std: "c11".to_string(),
            },
            memory: Memory {
                leak_check: "".to_string(),
                show_leak_kinds: "".to_string(),
                track_origins: false,
            },
        };
        assert_eq!(
            generate_build_command(&includes, &config, "src/main.c"),
            "gcc src/main.c c_target/debug/obj/5868638564572808266.o c_target/debug/obj/10537904563806491211.o -o c_target/debug/test -O0 -g -std=c11 "
        );
    }
}
