#![feature(ascii_char)]

//mod commands;
mod api;
mod gmail;
mod terminal;
mod util;

fn main() {
    gmail::get_credentials();
}
