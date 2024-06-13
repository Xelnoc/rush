use std::env;
use std::path::Path;

pub fn main(command:&str, args:&str,) {
    if command == "cd" {
        let path = Path::new(&env::current_dir().unwrap());
        if args == ".." {
            env::set_current_dir(Path::new(env::current_dir().unwrap().parent().unwrap()))
                .expect("unable to access directory {path.to_str()}");
        }
    } else if command == "cat" {

    } else {
        println!("command '{command}' does not exist")
    }
}