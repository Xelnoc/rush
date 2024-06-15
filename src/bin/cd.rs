use std::env;
use std::path::{Path, PathBuf};
use std::process::exit;


pub fn cd(args:&str) {
    if args == ".." {
        let path = env::current_dir().unwrap();
        let path = path.parent()
            .unwrap_or(&path);
        env::set_current_dir(Path::new(path)).unwrap();
    } else if args == "" {
        env::set_current_dir(env::home_dir().unwrap()).unwrap_or(());
    } else {
        let mut path = PathBuf::from(&env::current_dir().unwrap());
        path.push(args);
        env::set_current_dir(&path).unwrap_or(());
    }
}

fn main() {
    exit(0)
}