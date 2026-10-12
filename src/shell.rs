static _COMMANDS: &[&str] = &[
    "clear", "exit", "logs", "ls"
];

use std::io::{Write, stdin, stdout};
use std::process::Command;

pub fn init(args: Vec<String>) {
    println!("Initializing shell!");
}

pub fn prompt() -> Result<(), String> {
    print!("> ");
    stdout().flush();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let command = input.trim();

    if _COMMANDS.iter().any(|&c| c == command) {
        
        let command_result = Command::new(
            command.trim()
        ).spawn();

        match command_result {
            Ok(mut child) => {child.wait().expect("Couldn't run command!"); Ok(())},
            Err(_) => Err(format!("\"{command}\" is not a recognized command!")),
        }
    } else {
        Err(format!("\"{command}\" not in the list of supported commands!"))
    }
}