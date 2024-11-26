mod email;

use email::EmailCommands;

use clap::{Parser, Subcommand};

trait Command {
    fn run();
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
enum Commands {
    Email(EmailCommands),
}
