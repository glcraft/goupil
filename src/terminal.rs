use std::{fmt::Display, io};

pub fn ask_user(msg: impl Display) -> Option<String> {
    print!("{msg}: ");
    let mut result = String::new();
    if let Err(_) = io::stdin().read_line(&mut result) {
        return None;
    }
    return Some(result);
}
