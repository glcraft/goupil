mod commands;
mod credentials;
mod gmail;
mod secrets;
mod terminal;

use clap::Parser;

fn main() {
    simple_logger::SimpleLogger::new()
        .env()
        .init()
        .expect("unable to init simple_logger");

    let args = commands::Args::parse();
    args.command.run();
    // let api_config = secrets::ApiConfig::load();
    // gmail::get_credentials(&api_config);
}
