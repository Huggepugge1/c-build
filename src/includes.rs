use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum IncludeType {
    System,
    Local,
}

#[derive(Debug, Clone)]
pub struct Include {
    pub name: String,
    pub r#type: IncludeType,
}

fn open_file(path: &PathBuf, name: &str) -> Result<File, std::io::Error> {
    File::open(PathBuf::from(format!(
        "{}/{}",
        path.to_str().unwrap(),
        name
    )))
}

fn get_includes_from_file(
    path: &PathBuf,
    name: &str,
    already_included: &mut Vec<String>,
) -> Result<Vec<Include>, std::io::Error> {
    if already_included.contains(&name.to_string()) {
        return Ok(Vec::new());
    }
    already_included.push(name.to_string());

    let mut includes = Vec::new();

    let file = match open_file(path, name) {
        Ok(file) => file,
        Err(e) => {
            return Err(std::io::Error::new(
                e.kind(),
                format!("{}   {}/{}", e.to_string(), path.to_str().unwrap(), name),
            ))
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

            let name = &include[1..include.len() - 1];

            if already_included.contains(&name.to_string()) {
                line.clear();
                continue;
            }

            if include.starts_with("\"") {
                includes.push(Include {
                    name: name.to_string(),
                    r#type: IncludeType::Local,
                });
                includes.append(&mut get_includes_from_file(path, name, already_included)?);
                includes.append(&mut get_includes_from_file(
                    path,
                    &(name[0..name.len() - 1].to_string() + "c"),
                    already_included,
                )?);
            } else {
                includes.push(Include {
                    name: name.to_string(),
                    r#type: IncludeType::System,
                });
                already_included.push(name.to_string());
            }
        }
        line.clear();
    }
    Ok(includes)
}

pub fn get_includes(path: PathBuf) -> Result<Vec<Include>, std::io::Error> {
    get_includes_from_file(&path, "main.c", &mut Vec::new())
}
