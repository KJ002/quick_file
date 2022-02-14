use std::{fs::{write, read_to_string}, panic::panic_any};

pub struct ConfigManager {
    pub name: String,
    pub root_path: String
}

impl ConfigManager {
    pub fn new(name: String) -> Self {
        Self {
            name: name.clone(),
            root_path: match std::env::consts::OS {
                "linux" | "macos" => match std::env::var("HOME") {
                    Ok(x) => format!("{}/.{}", x, name),
                    Err(err) => panic_any(err),
                },
                "windows" => match std::env::var("APPDATA") {
                    Ok(x) => format!("{}/{}", x, name),
                    Err(err) => panic_any(err),
                },
                _ => unimplemented!(),
            }
        }
    }

    pub fn write(&self, file_name: &str, content: &str) {
        write(format!("{}/{}", self.root_path, file_name), content).unwrap()
    }

    pub fn read(&self, file_name: &str) -> String {
        let path = format!("{}/{}", self.root_path, file_name);
        match read_to_string(path) {
            Ok(content) => content,
            Err(err) => panic_any(err)
        }
    }

    pub fn swap_file(&self, file_name: &str, content: &str) -> String {
        let previous_content = self.read(file_name);
        self.write(file_name, content);
        previous_content
    }
}
