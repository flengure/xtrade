// src/bin/sandbox.rs

fn main() {
    println!("This is the sandbox!");
    check_config_file();
}

// Example function to test
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn check_config_file() {
    let file_path = "config.toml";
    if std::fs::metadata(file_path).is_ok() {
        println!("Config file found: {}", file_path);
    } else {
        println!("Config file not found: {}", file_path);
    }
}
