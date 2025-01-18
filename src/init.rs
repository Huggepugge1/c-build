use crate::cli::Init;

use crate::test::test_framework::create_test_framework;

const SRC_DIR: &str = "src/";
const MAIN_FILE: &str = "src/main.c";

const MAIN_FILE_CONTENTS: &str = "#include <stdio.h>

int main() {
    printf(\"Hello, World!\\n\");
    return 0;
}";

fn create_dir(path: &str) -> Result<(), String> {
    if !std::path::Path::new(path).exists() {
        match std::fs::create_dir(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to create directory: {}", e)),
        }
    } else {
        Ok(())
    }
}

fn create_main_file(path: &str) -> Result<(), String> {
    let src_path = format!("{}/{}", path, SRC_DIR);

    create_dir(&src_path)?;

    let main_file = format!("{}/{}", path, MAIN_FILE);

    match std::fs::write(main_file, MAIN_FILE_CONTENTS) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to create main file: {}", e)),
    }
}

fn create_git_repo(path: &str) -> Result<(), String> {
    let command = format!("cd {} && git init", path);

    match crate::command::output(&command) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to create git repo: {}", e)),
    }
}

fn create_git_ignore(path: &str) -> Result<(), String> {
    let git_ignore = format!("{}/.gitignore", path);
    let contents = "c_target/";

    match std::fs::write(git_ignore, contents) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to create git ignore: {}", e)),
    }
}

fn create_toml(args: &Init) -> Result<(), String> {
    let toml = format!("{}/c-build.toml", args.path);
    let name = if args.path == "." {
        std::env::current_dir()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    } else {
        args.path.split("/").last().unwrap().to_string()
    };

    let mut contents = "[package]\n".to_string();
    contents.push_str(&format!("name = \"{}\"\n", name));
    contents.push_str("version = \"0.1.0\"\n");
    contents.push_str("authors = [\"Your Name\"]\n");
    contents.push_str("src = \"src\"\n\n");

    contents.push_str("[debug]\n");
    contents.push_str("path = \"c_target/debug/\"\n");
    contents.push_str("debug = true\n");
    contents.push_str("optimization = 0\n");
    contents.push_str("warnings = true\n");
    contents.push_str("pedantic = true\n");
    contents.push_str("std = \"c2x\"\n\n");

    contents.push_str("[release]\n");
    contents.push_str("path = \"c_target/release/\"\n");
    contents.push_str("debug = false\n");
    contents.push_str("optimization = 3\n");
    contents.push_str("warnings = true\n");
    contents.push_str("pedantic = false\n");
    contents.push_str("std = \"c2x\"\n\n");

    contents.push_str("[memory]\n");
    contents.push_str("leak_check = \"full\"\n");
    contents.push_str("show_leak_kinds = \"all\"\n");
    contents.push_str("track_origins = true\n");

    match std::fs::write(toml, contents) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to create toml: {}", e)),
    }
}

pub fn init(args: &Init) -> Result<String, String> {
    create_dir(&args.path)?;
    create_main_file(&args.path)?;
    create_git_repo(&args.path)?;
    create_git_ignore(&args.path)?;
    create_toml(args)?;
    create_test_framework(&args.path)?;

    Ok("Initialized project".to_string())
}
