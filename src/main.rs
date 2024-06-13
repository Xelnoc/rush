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
    let mut dir: String = env::var("HOME").unwrap();
    loop {
        print!("rush {} | ", &dir);
        io::stdout().flush().expect("unable to flush");
        let input = readline();
        let command: &str = input.substring(0, input.as_str().chars().position(|x: char| x.to_string() == " ").unwrap_or(input.len() - 1) + 1);
        let args: &str = input.substring(input.as_str().chars().position(|x: char| x.to_string() == " ").unwrap() + 1, input.len() - 1);
        println!("{args}");
        exec::main(&command, &args, dir.clone())
    }
}
