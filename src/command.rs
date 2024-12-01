use std::process::exit;

pub fn handle_cmd(command: &str) {
    match command {
        "exit 0" => {
            exit(0);
        }
        _ => {
            println!("{command}: command not found");
        }
    }
}
