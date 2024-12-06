use std::io::{self, Read};

pub fn get_stdin() -> String {
    let mut string = String::new();
    let _ = io::stdin().read_to_string(&mut string);
    return string.trim().to_string();
}
