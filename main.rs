use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, exit};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).expect("Failed to read line");
        let trimmed_input = input.trim();

        if trimmed_input == "exit 0" {
            exit(0);
        } else if trimmed_input == "pwd" {
            match env::current_dir() {
                Ok(current_dir) => println!("{}", current_dir.display()),
                Err(_) => println!("Error retrieving current directory"),
            }
        } else if trimmed_input.starts_with("cd ") {
            let path = &trimmed_input[3..];
            match cd_command(path) {
                Ok(_) => (),
                Err(err) => println!("{}", err),
            }
        } else if trimmed_input.starts_with("type ") {
            let cmd_to_check = &trimmed_input[5..];
            match cmd_to_check {
                "echo" | "exit" | "type" | "pwd" | "cd" => {
                    println!("{} is a shell builtin", cmd_to_check);
                }
                _ => {
                    if let Some(path) = find_executable(cmd_to_check) {
                        println!("{} is {}", cmd_to_check, path);
                    } else {
                        println!("{}: not found", cmd_to_check);
                    }
                }
            }
        } else if trimmed_input.starts_with("echo ") || trimmed_input == "echo" {
            let message = &trimmed_input[5..];
            println!("{}", message);
        } else {
            let mut parts = trimmed_input.split_whitespace();
            if let Some(program) = parts.next() {
                if let Some(path) = find_executable(program) {
                    let args: Vec<&str> = parts.collect();
                    let result = Command::new(path)
                        .args(&args)
                        .output();

                    match result {
                        Ok(output) => {
                            print!("{}", String::from_utf8_lossy(&output.stdout));
                            io::stdout().flush().unwrap();
                        }
                        Err(e) => {
                            println!("Error running command: {}", e);
                        }
                    }
                } else {
                    println!("{}: command not found", program);
                }
            }
        }

        input.clear();
    }
}

fn cd_command(path: &str) -> Result<(), String> {
    let new_path = if path.starts_with('/') {
        PathBuf::from(path) // Absolute path
    } else if path == "~" {
        env::var("HOME").map(PathBuf::from).map_err(|e| e.to_string())? // Home directory
    } else {
        env::current_dir()
            .map_err(|e| e.to_string())?
            .join(path) // Relative path
    };

    if new_path.exists() && new_path.is_dir() {
        env::set_current_dir(&new_path).map_err(|e| e.to_string())
    } else {
        Err(format!("cd: {}: No such file or directory", path))
    }
}

fn find_executable(cmd: &str) -> Option<String> {
    if let Ok(paths) = env::var("PATH") {
        for path in paths.split(':') {
            let full_path = format!("{}/{}", path, cmd);
            if Path::new(&full_path).exists() {
                return Some(full_path);
            }
        }
    }
    None
}
