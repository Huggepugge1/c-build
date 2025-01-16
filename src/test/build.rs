use super::test_framework;
use crate::command::output;

fn generate_build_command() -> String {
    String::from(
        "gcc -fdiagnostics-color=always -g -O0 -Wall -pedantic -o c_target/test/test tests/tests.c tests/test_framework.c",
    )
}

pub fn build() -> Result<Option<String>, String> {
    test_framework::write_tests_to_file();

    let target_dir = "c_target/test";
    match std::fs::create_dir_all(target_dir) {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to create target directory: {}", e)),
    }

    println!("Building tests...");
    let command = generate_build_command();
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
