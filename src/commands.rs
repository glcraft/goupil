mod email;

use email::EmailArgs;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
#[non_exhaustive]
pub enum Commands {
    Email(EmailArgs),
}

impl Commands {
    pub fn run(&self) {
        match self {
            Self::Email(c) => c.command.run(),
            _ => unimplemented!("not implemented yet"),
        }
    }
}
