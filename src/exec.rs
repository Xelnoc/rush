use std::env;
use std::path::{Path, PathBuf};

pub fn main(command:&str, args:&str,) {
    if command == "cd" {
        if args == ".." {
            let path = env::current_dir().unwrap();
            let path = path.parent()
                .unwrap_or(&path);
            env::set_current_dir(Path::new(path))
                .unwrap_or(println!("unable to access {}", path.display()));
        } else {
            let mut path = PathBuf::from(&env::current_dir().unwrap());
            path.push(args);
            env::set_current_dir(&path)
                .unwrap_or(println!("unable to access {}", path.display()));
        }
    } else if command == "cat" {

    } else {
        println!("command '{command}' does not exist")
    }
}