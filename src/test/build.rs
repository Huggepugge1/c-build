use super::test_framework;
use crate::command::output;
use crate::includes::{get_includes_from_file, Include};

fn generate_build_command(includes: Vec<Include>) -> String {
    let mut command = String::from(
        "gcc -fdiagnostics-color=always -g -O0 -Wall -pedantic -o c_target/test/test tests/tests.c ",
    );

    for include in includes {
        match include.kind {
            crate::includes::IncludeType::Local(path) => {
                let path = path.with_extension("c");
                let path = path.to_str().unwrap();
                command.push_str(path);
                command.push(' ');
            }
            crate::includes::IncludeType::System => (),
        }
    }

    command
}

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

pub fn build() -> Result<Option<String>, String> {
    println!("Building tests...");

    test_framework::write_tests_to_file();

    let target_dir = "c_target/test";
    match std::fs::create_dir_all(target_dir) {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to create target directory: {}", e)),
    }

    let includes = get_test_includes();

    let command = generate_build_command(includes);
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
