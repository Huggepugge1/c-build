use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;
use std::process::exit;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IncludeType {
    System,
    Local(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Include {
    pub kind: IncludeType,
}

fn has_c_file(path: &PathBuf, name: &str) -> bool {
    let file = open_file(&path.join(name).with_extension("c"));
    file.is_ok()
}

fn open_file(path: &PathBuf) -> Result<File, std::io::Error> {
    File::open(path)
}

fn get_includes_from_file(
    path: &PathBuf,
    name: &str,
    already_included: &mut Vec<String>,
) -> Vec<Include> {
    if already_included.contains(&name.to_string()) {
        return Vec::new();
    }
    already_included.push(name.to_string());

    let mut includes = Vec::new();

    let file = match open_file(&path.join(name)) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to open file: {}/{}", path.to_str().unwrap(), name);
            exit(1);
        }
    };

    let mut contents = std::io::BufReader::new(file);

    let mut line: String = String::new();

    while match contents.read_line(&mut line) {
        Ok(0) => false,
        Ok(_) => true,
        Err(_) => false,
    } {
        if line.starts_with("#include") {
            let include = line
                .clone()
                .strip_prefix("#include")
                .unwrap()
                .trim()
                .to_string();

            let mut name = &include[1..include.len() - 1];

            if already_included.contains(&name.to_string()) {
                line.clear();
                continue;
            }

            if include.starts_with("\"") {
                let name_as_path = PathBuf::from(name);

                let relative_path = if let Some(parent) = name_as_path.parent() {
                    name = name_as_path.file_name().unwrap().to_str().unwrap();
                    path.join(parent)
                } else {
                    path.to_path_buf()
                }
                .canonicalize()
                .unwrap();

                if !has_c_file(&relative_path, name) {
                    if !already_included.contains(&name.to_string()) {
                        println!(
                            "Note: Included header file `{}/{}`, has no corresponding source file",
                            relative_path.to_str().unwrap(),
                            name
                        );
                        already_included.push(name.to_string());
                    }
                    line.clear();
                    continue;
                }

                includes.push(Include {
                    kind: IncludeType::Local(relative_path.join(name)),
                });

                includes.append(&mut get_includes_from_file(
                    &relative_path,
                    name,
                    already_included,
                ));

                includes.append(&mut get_includes_from_file(
                    &relative_path,
                    PathBuf::from(name).with_extension("c").to_str().unwrap(),
                    already_included,
                ));
            } else {
                includes.push(Include {
                    kind: IncludeType::System,
                });
                already_included.push(name.to_string());
            }
        }
        line.clear();
    }
    includes
}

pub fn get_includes(path: PathBuf) -> Vec<Include> {
    let mut includes = get_includes_from_file(&path, "main.c", &mut Vec::new());
    includes.sort();
    includes.dedup();
    includes
}
