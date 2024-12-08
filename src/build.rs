use crate::cli;
use crate::command;
use crate::includes::{get_includes, Include, IncludeType};

use serde::Deserialize;
use std::fs;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub package: Package,
    pub debug: Build,
    pub release: Build,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    #[allow(dead_code)]
    version: String,
    #[allow(dead_code)]
    authors: Vec<String>,
    src: String,
}

#[derive(Debug, Deserialize)]
pub struct Build {
    pub path: String,
    debug: bool,
    optimization: u8,
    warnings: bool,
    pedantic: bool,
    std: String,
}

pub fn get_build_options() -> Result<Config, String> {
    let toml = match fs::read_to_string("c-build.toml") {
        Ok(toml) => toml,
        Err(e) => return Err(format!("Failed to read toml: {}", e)),
    };
    let build: Config = match toml::from_str(&toml) {
        Ok(build) => build,
        Err(e) => return Err(format!("Failed to parse toml: {}", e)),
    };
    Ok(build)
}

fn generate_build_command(includes: &Vec<Include>, package: &Package, build: &Build) -> String {
    let mut command = format!("gcc {}main.c ", package.src);
    for include in includes {
        match include.r#type {
            IncludeType::Local => {
                command.push_str(&format!(
                    "{}obj/{} ",
                    build.path,
                    include.name.replace(".h", ".o")
                ));
            }
            IncludeType::System => (),
        }
    }

    command.push_str(&format!("-o {}{} ", build.path, package.name));
    command.push_str(&format!("-O{} ", build.optimization));

    if build.debug {
        command.push_str("-g ");
    }
    if build.warnings {
        command.push_str("-Wall ");
    }
    if build.pedantic {
        command.push_str("-pedantic ");
    }
    command.push_str(&format!("-std={} ", build.std));
    command
}

fn create_output_directory(build: &Build) {
    let path = std::path::PathBuf::from(build.path.clone());
    if !path.exists() {
        fs::create_dir_all(path).expect("Failed to create output directory");
    }
    let obj_path = std::path::PathBuf::from(format!("{}obj", build.path));
    if !obj_path.exists() {
        fs::create_dir_all(obj_path).expect("Failed to create output directory");
    }
}

fn should_build(include: &Include, package: &Package, build: &Build) -> Result<bool, String> {
    let metadata = match fs::metadata(format!(
        "{}{}",
        package.src,
        include.name.replace(".h", ".c")
    )) {
        Ok(metadata) => metadata,
        Err(e) => {
            return Err(format!(
                "Failed to fetch metadata for file: {}",
                e.to_string()
            ))
        }
    };
    if let Ok(created_time) = metadata.modified() {
        match fs::metadata(format!(
            "{}obj/{}",
            build.path,
            include.name.replace(".h", ".o")
        )) {
            Ok(metadata) => {
                if let Ok(obj_created_time) = metadata.modified() {
                    return Ok(created_time > obj_created_time);
                }
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => return Ok(true),
                _ => {
                    return Err(format!(
                        "Failed to fetch metadata for file: {}",
                        e.to_string()
                    ))
                }
            },
        };
    }
    Ok(true)
}

fn build_object(
    include: &Include,
    package: &Package,
    build: &Build,
) -> Result<Option<String>, String> {
    if !should_build(include, package, build)? {
        return Ok(None);
    }
    let command = format!(
        "gcc -c {}{} -o {}obj/{}",
        package.src,
        include.name.replace(".h", ".c"),
        build.path,
        include.name.replace(".h", ".o")
    );
    match command::output(&command) {
        Ok(output) => {
            if !output.status.success() {
                return Err(format!(
                    "Failed to build object file: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
        Err(e) => return Err(format!("Failed to run command: {}", e)),
    }
    Ok(None)
}

fn build_object_files(
    includes: &Vec<Include>,
    package: &Package,
    build: &Build,
) -> Result<Option<String>, String> {
    for include in includes {
        match include.r#type {
            IncludeType::Local => match build_object(include, package, build) {
                Ok(Some(v)) => println!("{}", v),
                Ok(None) => (),
                Err(e) => return Err(e),
            },
            IncludeType::System => (),
        }
    }
    Ok(None)
}

pub fn build(build: &cli::Build) -> Result<Option<String>, String> {
    let config: Config = match get_build_options() {
        Ok(config) => config,
        Err(e) => return Err(e),
    };
    let package = config.package;
    let build = if build.release {
        config.release
    } else {
        config.debug
    };

    let path = std::path::PathBuf::from(&package.src);
    let includes = match get_includes(path) {
        Ok(includes) => includes,
        Err(error) => {
            eprintln!("Failed to fetch dependencies: {}", error);
            std::process::exit(1);
        }
    };

    let build_command = generate_build_command(&includes, &package, &build);

    create_output_directory(&build);
    build_object_files(&includes, &package, &build)?;

    println!("Building {}{}", build.path, package.name);
    match command::output(&build_command) {
        Ok(output) => {
            if !output.status.success() {
                Err(format!(
                    "Failed to build: {}",
                    String::from_utf8_lossy(&output.stderr)
                ))
            } else {
                Ok(Some("Build successful".to_string()))
            }
        }
        Err(e) => Err(format!("Failed to run command: {}", e)),
    }
}
