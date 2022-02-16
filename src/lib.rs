use std::{
    fs::{read_to_string, write},
    panic::panic_any,
};

pub struct ConfigManager {
    pub root_path: String,
}

impl ConfigManager {
    pub fn new(name: &str) -> Self {
        Self {
            root_path: match std::env::consts::OS {
                "linux" | "macos" => match std::env::var("HOME") {
                    Ok(home) => format!("{}/.{}", home, name),
                    Err(err) => panic_any(err),
                },
                "windows" => match std::env::var("APPDATA") {
                    Ok(appdata) => format!("{}/{}", appdata, name),
                    Err(err) => panic_any(err),
                },
                _ => unimplemented!(),
            },
        }
    }

    pub fn write(&self, file_name: &str, content: &str) -> std::io::Result<()> {
        write(format!("{}/{}", self.root_path, file_name), content)
    }

    pub fn read(&self, file_name: &str) -> std::io::Result<String> {
        let path = format!("{}/{}", self.root_path, file_name);
        read_to_string(path)
    }

    pub fn swap_file(&self, file_name: &str, content: &str) -> std::io::Result<String> {
        let previous_content = self.read(file_name);
        self.write(file_name, content)?;
        previous_content
    }
}
