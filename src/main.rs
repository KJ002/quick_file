use std::{fs::{File, read_to_string}, io::Write, panic::panic_any};

const BINARY_NAME: &str = env!("CARGO_BIN_NAME");

fn get_path(file_name: &str) -> String {
    match std::env::consts::OS {
        "linux" | "macos" => match std::env::var("HOME") {
            Ok(x) => format!("{}/.{}/{}", x, BINARY_NAME, file_name),
            Err(err) => panic_any(err),
        },
        "windows" => match std::env::var("APPDATA") {
            Ok(x) => format!("{}/{}/{}", x, BINARY_NAME, file_name),
            Err(err) => panic_any(err),
        },
        _ => unimplemented!(),
    }
}

fn get_file(file_name: &str) -> File {
    let path = get_path(file_name);

    match File::open(&path) {
        Ok(handle) => handle,
        Err(_) => File::create(path).expect("Could not create file."),
    }
}

fn write_file(file_name: &str, content: String) -> std::io::Result<usize> {
    let buffer: &[u8] = content.as_bytes();
    get_file(file_name).write(buffer)
}

fn main() {
    get_file("test.dat");
    println!("Hello, world!");
}
