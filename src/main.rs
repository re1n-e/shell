use shell::command;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    let mut input = String::new();
    print!("$ ");
    io::stdout().flush().unwrap();
    while io::stdin().read_line(&mut input).is_ok() {
        command::handle_cmd(&input.trim());
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }

    // loop {
    //     print!("$ ");
    //     io::stdout().flush().unwrap();

    //     // Wait for user input
    //     let stdin = io::stdin();
    //     let mut input = String::new();
    //     stdin.read_line(&mut input).unwrap();
    //     command::handle_cmd(&input.trim());
    // }
}
