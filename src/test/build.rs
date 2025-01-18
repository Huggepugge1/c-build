use super::test_framework;

use crate::build::{
    build_object_files, create_output_directory, generate_build_command, get_build_options,
};
use crate::cli::Build;
use crate::command::output;
use crate::includes::{get_includes_from_file, Include};

fn get_test_includes() -> Vec<Include> {
    let mut includes = Vec::new();
    let mut include_strings = Vec::new();
    let tests = test_framework::get_tests();
    for test in tests.test_files {
        includes.append(&mut get_includes_from_file(
            test.parent().unwrap(),
            test.file_name().unwrap().to_str().unwrap(),
            &mut include_strings,
        ));
        includes.append(&mut get_includes_from_file(
            test.parent().unwrap(),
            test.file_name().unwrap().to_str().unwrap(),
            &mut include_strings,
        ));
    }
    includes.sort();
    includes.dedup();
    includes
}

pub fn build(build: &Build) -> Result<Option<String>, String> {
    let config = get_build_options(build)?;

    println!("Building tests...");
    create_output_directory(&config)?;
    test_framework::write_tests_to_file();

    let includes = get_test_includes();
    build_object_files(&includes, &config)?;

    let main_file = "tests/tests.c";
    let command = generate_build_command(&includes, &config, main_file);

    match output(&command) {
        Ok(status) => {
            if status.success() {
                Ok(Some(String::from("Tests built successfully")))
            } else {
                Err(String::from("Failed to build tests"))
            }
        }
        Err(e) => Err(format!("Failed to build tests: {}", e)),
    }
}
