use crate::cli::Build;
use crate::command;
use crate::file;
use crate::file::{Include, IncludeType};

use std::fs;

struct CompilerFlags {
    path: String,
    output: String,
    debug: bool,
    optimization: u8,
    warnings: bool,
    pedantic: bool,
    std: String,
}

impl CompilerFlags {
    fn new(args: &Build) -> Self {
        Self {
            path: args.path.clone(),
            output: args.output.clone(),
            debug: args.debug,
            optimization: args.optimization,
            warnings: args.warnings,
            pedantic: args.pedantic,
            std: args.std.clone(),
        }
    }
}

fn generate_build_command(includes: &Vec<Include>, flags: &CompilerFlags) -> String {
    let mut command = format!("gcc {}/main.c ", flags.path);
    for include in includes {
        match include.r#type {
            IncludeType::Local => {
                command.push_str(&format!(
                    "c_target/obj/{} ",
                    include.name.replace(".h", ".o")
                ));
            }
            IncludeType::System => (),
        }
    }

    command.push_str(&format!("-o {} ", flags.output));
    command.push_str(&format!("-O{} ", flags.optimization));

    if flags.debug {
        command.push_str("-g ");
    }
    if flags.warnings {
        command.push_str("-Wall ");
    }
    if flags.pedantic {
        command.push_str("-pedantic ");
    }
    command.push_str(&format!("-std={} ", flags.std));
    command
}

fn create_output_directory() {
    let path = std::path::PathBuf::from("c_target");
    if !path.exists() {
        fs::create_dir_all(path).expect("Failed to create output directory");
    }
    let obj_path = std::path::PathBuf::from("c_target/obj");
    if !obj_path.exists() {
        fs::create_dir_all(obj_path).expect("Failed to create output directory");
    }
}

fn should_build(path: &str, include: &Include) -> Result<bool, String> {
    let metadata = match fs::metadata(format!("{}/{}", path, include.name.replace(".h", ".c"))) {
        Ok(metadata) => metadata,
        Err(e) => {
            return Err(format!(
                "Failed to fetch metadata for file: {}",
                e.to_string()
            ))
        }
    };
    if let Ok(created_time) = metadata.modified() {
        match fs::metadata(format!("c_target/obj/{}", include.name.replace(".h", ".o"))) {
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

fn build_object(path: &str, include: &Include) -> Result<Option<String>, String> {
    if !should_build(path, include)? {
        return Ok(None);
    }
    let command = format!(
        "gcc -c {}/{} -o c_target/obj/{}",
        path,
        include.name.replace(".h", ".c"),
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

fn build_object_files(includes: &Vec<Include>, path: &str) -> Result<Option<String>, String> {
    for include in includes {
        match include.r#type {
            IncludeType::Local => match build_object(path, include) {
                Ok(Some(v)) => println!("{}", v),
                Ok(None) => (),
                Err(e) => return Err(e),
            },
            IncludeType::System => (),
        }
    }
    Ok(None)
}

pub fn build(args: &Build) -> Result<Option<String>, String> {
    let path = std::path::PathBuf::from(args.path.clone());
    let includes = match file::get_includes(path) {
        Ok(includes) => includes,
        Err(error) => {
            eprintln!("Failed to fetch dependencies: {}", error);
            std::process::exit(1);
        }
    };
    let flags = CompilerFlags::new(args);

    let build_command = generate_build_command(&includes, &flags);

    create_output_directory();
    build_object_files(&includes, &flags.path)?;

    println!("Building {}", args.output);
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
