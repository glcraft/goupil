use core::num;
use std::{
    fmt::Display,
    io::{self, Write},
};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ParseInt(num::ParseIntError),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<num::ParseIntError> for Error {
    fn from(value: num::ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

pub fn ask_user(msg: impl Display) -> Option<String> {
    print!("{msg}: ");
    let mut result = String::new();
    if let Err(_) = io::stdin().read_line(&mut result) {
        return None;
    }
    return Some(result);
}

pub fn choose<C, S>(msg: impl Display, choices: C) -> Result<isize, Error>
where
    C: AsRef<[S]>,
    S: Display,
{
    let len = choices.as_ref().len();
    let padding = len.ilog10();
    loop {
        for (i, v) in choices.as_ref().iter().enumerate().map(|(i, v)| (i + 1, v)) {
            println!("{i: >0$}: {v}", padding as _);
        }
        print!("{msg}: ");
        io::stdout().flush()?;
        let mut user_choice = String::new();
        io::stdin().read_line(&mut user_choice)?;
        let result = match user_choice.trim().parse::<isize>() {
            Ok(v) => v,
            Err(e) if matches!(e.kind(), num::IntErrorKind::InvalidDigit) => {
                println!("Bad number.");
                continue;
            }
            e => e?,
        };
        if !(result >= 1 && result <= len as isize) {
            println!("Wrong number. The number must be between 1 and {len} included.");
            continue;
        }
        return Ok(result - 1);
    }
}
