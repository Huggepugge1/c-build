use std::io::Write;
use std::path::PathBuf;

pub struct Tests {
    pub test_files: Vec<PathBuf>,
    pub tests: Vec<String>,
}

const TEST_FRAMEWORK_C: &str = r#"#include "test_framework.h"

#include <setjmp.h>

jmp_buf jmpbuf;
char *error_msg;

void fail(char *msg) {
    error_msg = msg;
    longjmp(jmpbuf, 1);
}
"#;

const TEST_FRAMEWORK_H: &str = r#"#include <string.h>
#include <math.h>

#pragma once
#define TEST(name) void test_##name()
#define TO_STRING(x) #x
#define STRINGIFY(x) TO_STRING(x)

#define ASSERT(cond)                                                           \
    if (!(cond)) {                                                             \
        fail(__FILE__ ":" STRINGIFY(__LINE__) ": " #cond);                     \
    }

#define ASSERT_EQ(a, b)                                                        \
    if ((a) != (b)) {                                                          \
        fail(__FILE__ ":" STRINGIFY(__LINE__) ": " #a " != " #b);              \
    }

#define ASSERT_STRING_EQ(a, b)                                                 \
    if (strcmp(a, b) != 0) {                                                   \
        fail(__FILE__ ":" STRINGIFY(__LINE__) ": " #a " != " #b);              \
    }

#define ASSERT_NULL(a)                                                         \
    if ((a) != NULL) {                                                         \
        fail(__FILE__ ":" STRINGIFY(__LINE__) ": " #a " != NULL");             \
    }

#define ASSERT_NOT_NULL(a)                                                     \
    if ((a) == NULL) {                                                         \
        fail(__FILE__ ":" STRINGIFY(__LINE__) ": " #a " == NULL");             \
    }
    
#define ASSERT_FALSE(a)                                                        \
    if (a) {                                                                   \
        fail(__FILE__ ":" STRINGIFY(__LINE__) ": " #a " is not false");        \
    }

#define ASSERT_FLOAT_EQ(a, b)                                                  \
    if (fabs((a) - (b)) > 1e-6) {                                              \
        fail(__FILE__ ":" STRINGIFY(__LINE__) ": " #a " != " #b);              \
    }

typedef void test_fn();

struct Test {
    const char *name;
    test_fn *test;
};

void fail(char *msg);
"#;

pub fn get_test_files() -> Vec<PathBuf> {
    let mut test_files = Vec::new();
    for file in std::fs::read_dir("tests").unwrap() {
        let file = file.unwrap();
        let path = file.path();
        match file.file_type().unwrap() {
            t if t.is_dir() => (),
            t if t.is_file() => {
                if path.extension().unwrap() == "c"
                    && file.file_name().into_string().unwrap() != "test_framework.c"
                    && file.file_name().into_string().unwrap() != "tests.c"
                {
                    test_files.push(path);
                }
            }
            _ => (),
        }
    }
    test_files
}

fn get_tests_from_files(test_files: Vec<PathBuf>) -> Tests {
    let mut tests = Vec::new();
    for file in &test_files {
        let file = std::fs::read_to_string(file).unwrap();
        for line in file.lines() {
            if line.starts_with("TEST(") {
                tests.push(
                    line.split("(").collect::<Vec<&str>>()[1]
                        .split(")")
                        .collect::<Vec<&str>>()[0]
                        .to_string(),
                );
            }
        }
    }
    Tests { test_files, tests }
}

pub fn get_tests() -> Tests {
    let test_files = get_test_files();
    get_tests_from_files(test_files)
}

pub fn write_tests_to_file() {
    let tests = get_tests();
    let test_count = tests.tests.len();
    let mut file = std::fs::File::create("tests/tests.c").unwrap();
    file.write_all("#include \"test_framework.h\"\n\n".as_bytes())
        .unwrap();

    for test_file in tests.test_files {
        file.write_all(
            format!("#include \"{}\"\n", test_file.to_str().unwrap())
                .replace("tests/", "")
                .as_bytes(),
        )
        .unwrap();
    }

    file.write_all("\n".as_bytes()).unwrap();
    file.write_all("#include <stdio.h>\n".as_bytes()).unwrap();
    file.write_all("#include <setjmp.h>\n".as_bytes()).unwrap();

    file.write_all("\n".as_bytes()).unwrap();
    file.write_all("extern jmp_buf jmpbuf;\n".as_bytes())
        .unwrap();
    file.write_all("extern char *error_msg;\n\n".as_bytes())
        .unwrap();

    file.write_all("struct Test tests[] = {\n".as_bytes())
        .unwrap();
    for test in tests.tests {
        file.write_all(format!("{{ \"{}\", test_{} }},\n", test, test).as_bytes())
            .unwrap();
    }

    file.write_all("};\n\n".as_bytes()).unwrap();

    file.write_all("int main() {\n".as_bytes()).unwrap();
    file.write_all(format!("for (int i = 0; i < {}; i++) {{\n", test_count).as_bytes())
        .unwrap();
    file.write_all("if (setjmp(jmpbuf) == 0) {\n".as_bytes())
        .unwrap();
    file.write_all("tests[i].test();\n".as_bytes()).unwrap();
    file.write_all("printf(\"Test `%s` passed\\n\", tests[i].name);\n".as_bytes())
        .unwrap();
    file.write_all("} else {\n".as_bytes()).unwrap();
    file.write_all("printf(\"Test `%s` failed: %s\\n\", tests[i].name, error_msg);\n".as_bytes())
        .unwrap();
    file.write_all("}\n".as_bytes()).unwrap();
    file.write_all("}\nreturn 0;\n}\n".as_bytes()).unwrap();
}

pub fn create_test_framework(path: &str) -> Result<(), String> {
    let dir = PathBuf::from(format!("{}/tests", path));
    std::fs::create_dir_all(&dir).unwrap();
    let test_framework_c = dir.join("test_framework.c");
    let test_framework_h = dir.join("test_framework.h");

    match std::fs::write(test_framework_c, TEST_FRAMEWORK_C) {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to create test framework source: {}", e)),
    }
    match std::fs::write(test_framework_h, TEST_FRAMEWORK_H) {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to create test framework header: {}", e)),
    };
    Ok(())
}
