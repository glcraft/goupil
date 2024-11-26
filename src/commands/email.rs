use crate::{commands::Command, terminal::ask_user};
use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum EmailCommands {
    Unsubscribe(EmailUnsubscribe),
}

#[derive(Args)]
pub struct EmailUnsubscribe {}

enum EmailKind {
    Gmail,
    Microsoft,
    Generic,
}

impl From<&String> for EmailKind {
    fn from(value: &String) -> Self {
        let domain = &value[value.find('@').expect("no '@' found")..];
        if domain.starts_with("gmail") {
            return Self::Gmail;
        }
        if domain.starts_with("hotmail") || domain.starts_with("live") {
            return Self::Microsoft;
        }
        return Self::Generic;
    }
}

impl Command for EmailUnsubscribe {
    fn run() {
        let email = ask_user("Enter your email");
    }
}
