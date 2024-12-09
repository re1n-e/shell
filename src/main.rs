#[allow(unused_imports)]
use std::io::{self, Write};

use std::{env, fs, iter::Peekable, path::PathBuf, process::Command, str::Chars};

const BUILTIN_COMMANDS: [&str; 5] = ["cd", "echo", "exit", "pwd", "type"];

fn get_env_var_value_unchecked(var: &str) -> String {
    env::vars()
        .find(|(v1, _)| v1 == var)
        .map(|(_, v2)| v2)
        .unwrap_or_default()
}

fn get_path_to_file(file_name: &str) -> Option<PathBuf> {
    let base_path = get_env_var_value_unchecked("PATH");

    let path_dirs = base_path.split(":");

    for path_dir in path_dirs {
        let fs_dirs = fs::read_dir(path_dir);

        if let Ok(dir) = fs_dirs {
            for dir_entry_res in dir {
                if let Ok(dir_entry) = dir_entry_res {
                    if dir_entry.file_name().to_str().unwrap_or("") == file_name {
                        return Some(dir_entry.path());
                    }
                }
            }
        }
    }

    None
}

#[derive(Debug, PartialEq, Eq)]

enum CommandKind {
    Cd,

    Echo,

    Exit,

    Unknown,

    Pwd,

    Type,
}

impl CommandKind {
    fn new(s: &str) -> Self {
        match s {
            "cd" => CommandKind::Cd,

            "echo" => CommandKind::Echo,

            "exit" => CommandKind::Exit,

            "pwd" => CommandKind::Pwd,

            "type" => CommandKind::Type,

            _ => CommandKind::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]

struct ShellCommand {
    kind: CommandKind,

    args: Vec<String>,
}

impl ShellCommand {
    fn new(kind: CommandKind, args: Vec<String>) -> Self {
        Self { kind, args }
    }
}

fn read_single_quoted_string(chars: &mut Peekable<Chars>, arg: &mut String) {
    let mut ch = chars.next().unwrap();

    while ch != '\'' {
        arg.push(ch);

        ch = chars.next().unwrap();
    }
}

fn read_double_quoted_string(chars: &mut Peekable<Chars>, arg: &mut String) {
    let mut ch = chars.next().unwrap();

    while ch != '"' {
        match ch {
            '\\' => {
                let peeked = chars.peek().unwrap();

                match peeked {
                    '\\' | '$' | '"' => {
                        arg.push(chars.next().unwrap());
                    }

                    _ => arg.push(ch),
                }
            }

            _ => arg.push(ch),
        }

        ch = chars.next().unwrap();
    }
}

fn parse_input_string(s: String) -> Option<ShellCommand> {
    let mut command_str = String::new();

    let mut chars = s.chars().peekable();

    let first_char = chars.next().unwrap();

    if first_char == '\n' {
        return None;
    }

    let mut ch = first_char;

    while ch.is_whitespace() {
        ch = chars.next().unwrap();
    }

    while !ch.is_whitespace() {
        if ch == '\'' {
            read_single_quoted_string(&mut chars, &mut command_str);
        } else if ch == '"' {
            read_double_quoted_string(&mut chars, &mut command_str);
        } else {
            command_str.push(ch);
        }

        ch = chars.next().unwrap();
    }

    let mut args: Vec<String> = vec![];

    // check if we have another char in the input. if not, we have no args

    let next_char = chars.next();

    match next_char {
        Some(c) => {
            ch = c;
        }

        None => {
            args.push(command_str.clone());

            return Some(ShellCommand::new(CommandKind::new(&command_str), args));
        }
    };

    let mut arg_string = String::new();

    while ch != '\n' {
        if ch == '\'' {
            read_single_quoted_string(&mut chars, &mut arg_string);
        } else if ch == '"' {
            read_double_quoted_string(&mut chars, &mut arg_string);
        } else if ch == ' ' {
            if arg_string.len() > 0 {
                args.push(arg_string);

                arg_string = String::new();
            }
        } else if ch == '\\' {
            ch = chars.next().unwrap();

            arg_string.push(ch);
        } else {
            arg_string.push(ch);
        }

        ch = chars.next().unwrap();
    }

    args.push(arg_string);

    let command_kind = CommandKind::new(&command_str);

    if command_kind == CommandKind::Unknown {
        args.insert(0, command_str.clone());
    }

    Some(ShellCommand::new(CommandKind::new(&command_str), args))
}

fn main() {
    let mut current_directory = env::current_dir().unwrap();

    let base_path = get_env_var_value_unchecked("HOME");

    let home = PathBuf::from(&base_path);

    loop {
        print!("$ ");

        io::stdout().flush().unwrap();

        // Wait for user input

        let stdin = io::stdin();

        let mut input = String::new();

        stdin.read_line(&mut input).unwrap();

        let shell_command_opt = parse_input_string(input.clone());

        if shell_command_opt.is_none() {
            continue;
        }

        let shell_command = shell_command_opt.unwrap();

        // println!("{:?}", shell_command);

        match shell_command.kind {
            CommandKind::Cd => {
                let search_path_opt = shell_command.args.get(0);

                if let None = search_path_opt {
                    // alias for home in some shells, unimplemented for us

                    continue;
                }

                let search_path = search_path_opt.unwrap();

                if search_path.starts_with("/") {
                    // absolute

                    if let Ok(b) = fs::exists(&search_path) {
                        if b {
                            current_directory = PathBuf::from(&search_path);
                        } else {
                            println!("{}: No such file or directory", search_path);
                        }
                    }
                }

                if search_path.starts_with(".") {
                    // relative

                    if search_path.starts_with("..") {
                        for _ in search_path.matches("..") {
                            current_directory.pop();
                        }
                    } else {
                        let dir_to_nav = &search_path[2..];

                        let mut pb = PathBuf::from(&current_directory);

                        pb.push(dir_to_nav);

                        if pb.exists() {
                            current_directory = pb;
                        } else {
                            println!("{}: No such file or directory", pb.to_str().unwrap_or(""));
                        }
                    }
                }

                if search_path.starts_with("~") {
                    current_directory = home.clone();
                }
            }

            CommandKind::Echo => {
                let mut out = String::new();

                for arg in shell_command.args {
                    out.push_str(&arg);

                    out.push(' ');
                }

                println!("{}", out.trim());
            }

            CommandKind::Exit => break,

            CommandKind::Pwd => {
                println!("{}", current_directory.to_str().unwrap_or(""));
            }

            CommandKind::Type => {
                for arg in shell_command.args {
                    if BUILTIN_COMMANDS.iter().any(|v| v == &arg) {
                        println!("{arg} is a shell builtin");
                    } else {
                        let maybe_file_path = get_path_to_file(&arg);

                        if let Some(file_path) = maybe_file_path {
                            println!("{} is {}", arg, file_path.to_str().unwrap_or(""));
                        } else {
                            println!("{}: not found", arg);
                        }
                    }
                }
            }

            _ => {
                let name = shell_command.args.get(0).unwrap();

                let rest = shell_command.args.get(1..).unwrap();

                let maybe_file_path = get_path_to_file(&name);

                match maybe_file_path {
                    Some(file_path) => match Command::new(file_path).args(rest).output() {
                        Ok(output) => {
                            io::stdout().write_all(&output.stdout).unwrap();

                            io::stderr().write_all(&output.stderr).unwrap();
                        }

                        Err(e) => {
                            eprint!("{}", e);
                        }
                    },

                    None => {
                        println!("{name}: command not found");
                    }
                }
            }
        }
    }
}

#[cfg(test)]

mod tests {

    use crate::{parse_input_string, read_double_quoted_string, CommandKind};

    // #[test]

    fn test_input_parser() {
        let s = String::from("echo hok'stuff'bok\n");

        let res = parse_input_string(s);

        assert!(res.is_some());

        let result = res.unwrap();

        assert_eq!(result.kind, CommandKind::Echo);

        assert_eq!(result.args.len(), 1);

        let s = String::from("echo 'hok  bonk'    blurp\n");

        let res = parse_input_string(s);

        assert!(res.is_some());

        let result = res.unwrap();

        assert_eq!(result.kind, CommandKind::Echo);

        assert_eq!(result.args.len(), 2);

        assert_eq!(result.args.get(0).unwrap(), "hok  bonk");

        assert_eq!(result.args.get(1).unwrap(), "blurp");
    }

    #[test]

    fn double_quoted() {
        let s = String::from("/tmp/qux/'f  \\92'\"");

        let mut chars = s.chars().peekable();

        let mut res = String::new();

        read_double_quoted_string(&mut chars, &mut res);

        assert_eq!(res, String::from("/tmp/qux/'f  \\92'"));
    }

    #[test]

    fn escapes() {
        let s = String::from("echo shell\\ \\ \\ \\ example\n");

        let res = parse_input_string(s).unwrap();

        assert_eq!(res.args.get(0).unwrap(), "shell    example");
    }
}
