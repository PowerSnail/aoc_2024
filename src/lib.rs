use std::{
    env,
    io::{self, Read},
};

pub fn get_stdin() -> String {
    let mut string = String::new();
    let _ = io::stdin().read_to_string(&mut string);
    return string.trim().to_string();
}

pub fn get_part() -> u8 {
    let args: Vec<_> = env::args().collect();
    args[1].parse::<u8>().unwrap()
}
