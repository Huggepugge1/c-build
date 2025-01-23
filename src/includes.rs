use std::fs::File;
use std::io::BufRead;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IncludeType {
    System,
    Local(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Include {
    pub kind: IncludeType,
}

fn has_c_file(path: &Path, name: &str) -> bool {
    let file = open_file(&path.join(name).with_extension("c"));
    file.is_ok()
}

fn open_file(path: &PathBuf) -> Result<File, String> {
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(e) => Err(format!("Failed to open file: {}", e)),
    }
}

fn get_include_from_line(line: &str) -> (String, String) {
    let include = line.strip_prefix("#include").unwrap().trim().to_string();
    let name = include[1..include.len() - 1].to_string();

    (include, name)
}

pub fn get_includes_from_file(
    path: &Path,
    name: String,
    already_included: &mut Vec<String>,
) -> Result<Vec<Include>, String> {
    if already_included.contains(&name) {
        return Ok(Vec::new());
    }
    already_included.push(name.clone());

    let mut includes = Vec::new();

    let file = open_file(&path.join(name))?;

    let mut contents = std::io::BufReader::new(file);

    let mut line: String = String::new();

    while match contents.read_line(&mut line) {
        Ok(0) => false,
        Ok(_) => true,
        Err(_) => false,
    } {
        if line.starts_with("#include") {
            let (include, mut name) = get_include_from_line(&line);

            if already_included.contains(&name.to_string()) {
                line.clear();
                continue;
            }

            if include.starts_with("\"") {
                let name_as_path = PathBuf::from(&name);

                let relative_path = match if let Some(parent) = name_as_path.parent() {
                    name = name_as_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string();
                    path.join(parent)
                } else {
                    path.to_path_buf()
                }
                .canonicalize()
                {
                    Ok(path) => path,
                    Err(_) => {
                        return Err(format!(
                            "Included header file `{}/{}` not found",
                            path.to_str().unwrap(),
                            name
                        ));
                    }
                };

                if !has_c_file(&relative_path, &name) {
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
                    kind: IncludeType::Local(relative_path.join(&name)),
                });

                includes.append(&mut get_includes_from_file(
                    &relative_path,
                    name.clone(),
                    already_included,
                )?);

                includes.append(&mut get_includes_from_file(
                    &relative_path,
                    PathBuf::from(&name)
                        .with_extension("c")
                        .to_string_lossy()
                        .to_string(),
                    already_included,
                )?);
            } else {
                includes.push(Include {
                    kind: IncludeType::System,
                });
                already_included.push(name.to_string());
            }
        }
        line.clear();
    }
    Ok(includes)
}

pub fn get_includes(path: PathBuf) -> Result<Vec<Include>, String> {
    let mut includes = get_includes_from_file(&path, "main.c".to_string(), &mut Vec::new())?;
    includes.sort();
    includes.dedup();
    Ok(includes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_includes() {
        let includes = get_includes(PathBuf::from("examples/tests/src")).unwrap();
        assert_eq!(includes.len(), 3);
        assert_eq!(
            includes.contains(&Include {
                kind: IncludeType::Local(
                    PathBuf::from("examples/tests/src/testing.c")
                        .canonicalize()
                        .unwrap()
                )
            }),
            true
        );
        assert_eq!(
            includes.contains(&Include {
                kind: IncludeType::Local(
                    PathBuf::from("examples/tests/src/test.h")
                        .canonicalize()
                        .unwrap()
                )
            }),
            true
        );
    }
}
