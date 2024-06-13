mod exec;

use std::env;
use std::io;
use std::io::Write;
use substring::Substring;
use std::str;

fn readline() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn main() {
    loop {
        print!("rush {}: {} | ", env::var("USER").unwrap(), env::current_dir().unwrap().display());
        io::stdout().flush().expect("unable to flush");
        let input = readline();
        let command: &str = input.substring(0, input.as_str().chars().position(|x: char| x.to_string() == " ").unwrap_or(input.len() - 1));
        let args: &str;
        if input.len() > command.len() + 1 {
            args = input.substring(input.as_str().chars().position(|x: char| x.to_string() == " ").unwrap() + 1, input.len() - 1);
        } else {
            args = " "
        }
        exec::main(&command, &args)
    }
}
