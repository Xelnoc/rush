use std::env;
use std::io;
use std::io::Write;

fn readline() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn exec(command:&str, args:&str) {

}

fn main() {
    let mut dir: String = env::var("HOME").unwrap();
    while true {
        let mut input = String::new();
        print!("rush {} | ", dir);
        io::stdout().flush();
        let input = readline();

    }
}
