use std::process::exit;

pub fn handle_cmd(command: &str) {
    if command.starts_with("echo") {
        println!("{}", &command[5..]);
    } else if command.starts_with("exit") {
        exit(0);
    } else {
        println!("{}: command not found", command);
    }
}
