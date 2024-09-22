use std::io;
use std::io::Write;

pub fn typing(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}