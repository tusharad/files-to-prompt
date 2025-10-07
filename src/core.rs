use crate::cli::AppConfig;
use glob::Pattern;
use std::collections::HashSet;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{fs, io::Error, path::PathBuf};

pub fn process(app_config: AppConfig) -> Result<(), String> {
    let paths = app_config.paths.clone();
    let visited = Arc::new(Mutex::new(HashSet::new()));
    let cont = Arc::new(Mutex::new(String::new()));
    let cont1 = Arc::clone(&cont);

    let _ = process_paths(paths, app_config.clone(), visited, cont, 0);

    let output_filename = mk_extension(&app_config);
    let content = cont1.lock().unwrap();
    let _ = fs::write(output_filename, content.clone());
    Ok(())
}

fn process_paths(
    paths: Vec<PathBuf>,
    _app_config: AppConfig,
    _visited: Arc<Mutex<HashSet<(u64, u64)>>>,
    _cont: Arc<Mutex<String>>,
    max_depth: i32,
) -> Result<(), String> {
    let mut handles = vec![];

    for _path in paths {
        let cont = Arc::clone(&_cont);
        let visited = Arc::clone(&_visited);
        let app_config = _app_config.clone();

        let handle =
            thread::spawn(move || process_path(&_path, &app_config, visited, cont, max_depth));
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}

fn process_path(
    path: &PathBuf,
    app_config: &AppConfig,
    _visited: Arc<Mutex<HashSet<(u64, u64)>>>,
    cont: Arc<Mutex<String>>,
    max_depth: i32,
) {
    match path.try_exists() {
        Ok(true) => {
            let new_path = &relative_to_absolute(path);
            let include_hidden = app_config.include_hidden;
            let is_path_hidden = is_hidden_unix(new_path);
            let is_ignore_match = check_ignore_match(new_path, &app_config.ignore);

            // skipping if file/directory is hidden
            // or matches ignore pattern
            // or not of given extension
            if is_path_hidden && !include_hidden {
                println!("Skipping hidden {:?}", new_path);
                return;
            }

            if is_ignore_match {
                println!("Ignoring {:?}", new_path);
                return;
            }

            if path.is_file() && does_extension_match(new_path, &app_config.extension) {
                let _ = match app_config.markdown {
                    true => store_as_markdown(cont, new_path, app_config.line_numbers),
                    _ => store_as_default(cont, new_path, app_config.line_numbers),
                };
            } else if path.is_dir() {
                if let Ok(metadata) = path.metadata() {
                    let inode = metadata.ino();
                    let dev = metadata.dev();
                    let key = (inode, dev);
                    let mut visited = _visited.lock().unwrap();
                    if !visited.insert(key) || max_depth > app_config.depth {
                        // directory already visited, skipping
                        return;
                    }
                } else {
                    //failed to get path metadata, returning just in case
                    return;
                }

                let entries = path.read_dir().expect("Cannot read dir");
                let mut new_paths: Vec<PathBuf> = vec![];
                for entry in entries {
                    match entry {
                        Ok(new_path) => new_paths.push(new_path.path()),
                        Err(_) => println!("Cannot read dir"),
                    }
                }
                let _ = process_paths(new_paths, app_config.clone(), _visited, cont, max_depth + 1);
            }
        }
        _ => println!("{:?} path does not exists or is not readable", path),
    }
}

fn ext_to_lang(ext: &str) -> &'static str {
    match ext {
        "py" => "python",
        "rs" => "rust",
        "js" => "javascript",
        "ts" => "typescript",
        "java" => "java",
        "c" => "c",
        "cpp" => "cpp",
        "sh" => "bash",
        "rb" => "ruby",
        "hs" => "haskell",
        "html" => "html",
        "css" => "css",
        "xml" => "xml",
        "json" => "json",
        "yaml" | "yml" => "yaml",
        _ => "",
    }
}

fn line_numbers(file_cont: &mut String) {
    let original = file_cont.clone();
    file_cont.clear();
    for (i, line) in original.lines().enumerate() {
        file_cont.push_str(&format!("{}:{}\n", i + 1, line))
    }
}

fn store_as_default(
    _cont: Arc<Mutex<String>>,
    path: &PathBuf,
    line_num: bool,
) -> Result<(), io::Error> {
    let mut file_cont = fs::read_to_string(path)?;
    if line_num {
        line_numbers(&mut file_cont);
    }
    let mut cont = _cont.lock().unwrap();
    cont.push_str(path.to_str().unwrap_or(""));
    cont.push_str("\n");
    cont.push_str(&file_cont);
    Ok(())
}

fn store_as_markdown(
    _cont: Arc<Mutex<String>>,
    path: &PathBuf,
    line_num: bool,
) -> Result<(), Error> {
    let mut file_cont = fs::read_to_string(path)?;
    let backticks = "```";
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let ss = format!("{}{}\n", backticks, ext_to_lang(ext));
    if line_num {
        line_numbers(&mut file_cont);
    }
    let mut cont = _cont.lock().unwrap();
    cont.push_str(path.to_str().unwrap_or(""));
    cont.push_str("\n");
    cont.push_str(&ss);
    cont.push_str(&file_cont);
    cont.push_str("```\n");
    Ok(())
}

fn mk_extension(app_config: &AppConfig) -> String {
    let output_filepath = &app_config.output;
    if app_config.markdown {
        PathBuf::from(output_filepath)
            .with_extension("md")
            .to_str()
            .unwrap_or(&output_filepath)
            .to_string()
    } else {
        output_filepath.to_string()
    }
}

fn is_hidden_unix(path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map_or(false, |s| s.starts_with('.') && s != "." && s != "..")
}

fn check_ignore_match(path: &PathBuf, patterns: &Vec<String>) -> bool {
    let path_str = path.to_str().unwrap_or("");

    patterns.iter().any(|pattern| {
        Pattern::new(&pattern)
            .map(|pat| pat.matches(path_str))
            .unwrap_or(false)
    })
}

fn does_extension_match(path: &PathBuf, mb_extension_list: &Option<Vec<String>>) -> bool {
    match mb_extension_list {
        None => true,
        Some(extension_list) => {
            if extension_list.is_empty() {
                true
            } else {
                path.extension().map_or(false, |ext| {
                    extension_list
                        .iter()
                        .any(|x| x == ext.to_str().unwrap_or(""))
                })
            }
        }
    }
}

fn relative_to_absolute(path: &PathBuf) -> PathBuf {
    match path.is_relative() {
        true => fs::canonicalize(&path).unwrap_or(path.to_path_buf()),
        _ => path.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_hidden_unix() {
        assert!(is_hidden_unix(&PathBuf::from(".hidden")));
        assert!(!is_hidden_unix(&PathBuf::from("visible")));
        assert!(!is_hidden_unix(&PathBuf::from(".")));
        assert!(!is_hidden_unix(&PathBuf::from("..")));
    }

    #[test]
    fn test_does_extension_match() {
        let path = PathBuf::from("main.rs");
        let exts = Some(vec!["rs".to_string(), "py".to_string()]);
        assert!(does_extension_match(&path, &exts));

        let other = PathBuf::from("main.txt");
        assert!(!does_extension_match(&other, &exts));

        assert!(does_extension_match(&other, &None));
    }

    #[test]
    fn test_check_ignore_match() {
        let path = PathBuf::from("src/lib.rs");
        let patterns = vec!["src/*".to_string()];
        assert!(check_ignore_match(&path, &patterns));
    }
}
