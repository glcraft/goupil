#![feature(ascii_char)]

//mod commands;
mod gmail;
mod oauth2;
mod secrets;
mod terminal;
mod util;

fn main() {
    let api_config = secrets::ApiConfig::load();
    gmail::get_credentials(&api_config);
}
