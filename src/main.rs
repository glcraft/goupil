#![feature(ascii_char)]

//mod commands;
mod gmail;
mod terminal;
mod util;

fn main() {
    gmail::get_credentials();
}
