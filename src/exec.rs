#[path = "bin/cd.rs"] mod cd;

pub fn main(command:&str, args:&str) {
    if command == "cd" {
        cd::main(args);
    } else if command == "cat" {


    } else {
        println!("command '{command}' does not exist")
    }
}