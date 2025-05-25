use std::io::{Write, stdout};

pub fn print(msg: &str) {
    print!("{}", msg);
    stdout().flush().unwrap();
}
