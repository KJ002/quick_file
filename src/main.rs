use std::{fs::{write, read_to_string}, panic::panic_any};

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

fn write_file(file_name: &str, content: &str) {
    write(get_path(file_name), content).unwrap()
}

fn read_file(file_name: &str) -> String {
    let path = get_path(file_name);
    match read_to_string(path) {
        Ok(content) => content,
        Err(err) => panic_any(err)
    }
}

fn main() {
    write_file("test.dat", "this is a test sdf \n test");
    println!("{}", read_file("test.dat"))
}
