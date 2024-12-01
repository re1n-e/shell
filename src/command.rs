use std::process::exit;

fn handle_type(type_str: &str) {
    match type_str {
        "echo" => println!("echo is a shell builtin"),
        "exit" => println!("exit is a shell builtin"),
        "type" => println!("type is a shell builtin"),
        "cat" => println!("cat is /bin/cat"),
        _ => println!("{}: not found", type_str),
    }
}

pub fn handle_cmd(command: &str) {
    if command.starts_with("echo") {
        if command.len() > 4 {
            println!("{}", command[5..].trim());
        } else {
            println!("");
        }
    } else if command.starts_with("exit") {
        exit(0);
    } else if command.starts_with("type") {
        if command.len() > 4 {
            handle_type(command[5..].trim());
        }
    } else {
        println!("{}: command not found", command);
    }
}
