use std::env;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;

fn handle_type(type_str: &str, path_env: &str) {
    match type_str {
        "echo" => println!("echo is a shell builtin"),
        "exit" => println!("exit is a shell builtin"),
        "type" => println!("type is a shell builtin"),
        "pwd" => println!("pwd is a shell builtin"),
        _ => {
            let split = &mut path_env.split(':');
            if let Some(path) =
                split.find(|path| std::fs::metadata(format!("{}/{}", path, type_str)).is_ok())
            {
                println!("{type_str} is {path}/{type_str}");
            } else {
                println!("{type_str}: not found");
            }
        }
    }
}

/// Finds the executable path for the given command name by searching the `PATH` environment variable.
/// Returns `None` if not found or not executable.
fn find_exe(name: &str) -> Option<PathBuf> {
    if let Ok(paths) = env::var("PATH") {
        for path in env::split_paths(&paths) {
            let exe_path = path.join(name);
            if exe_path.is_file() {
                return Some(exe_path);
            }
        }
    }
    None
}

fn exe(command: &str) {
    let command: Vec<_> = command.split_whitespace().collect();
    let cmd = command[0];
    let args = &command[1..];
    if let Some(path) = find_exe(cmd) {
        Command::new(path)
            .args(args)
            .status()
            .expect("failed to execute process");
        return;
    }
    println!("{}: command not found", cmd);
}

fn print_current_working_directory() {
    let path = env::current_dir().unwrap();
    println!("{}", path.display());
}

pub fn handle_cmd(command: &str, path_env: &str) {
    let trimmed_cmd = command.trim();
    match trimmed_cmd.split_whitespace().next() {
        Some("echo") => println!("{}", trimmed_cmd[5..].trim()),
        Some("exit") => exit(0),
        Some("type") => handle_type(&trimmed_cmd[5..].trim(), path_env),
        Some("pwd") => print_current_working_directory(),
        Some(_) => exe(trimmed_cmd),
        None => {} // Empty command, do nothing
    }
}
