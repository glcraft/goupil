use clap::{Args, Subcommand};

use crate::terminal;

#[derive(Args, Debug)]
pub struct EmailArgs {
    #[command(subcommand)]
    pub command: EmailCommands,
}

#[derive(Subcommand, Debug)]
pub enum EmailCommands {
    Unsubscribe(EmailUnsubscribeArgs),
}

impl EmailCommands {
    pub fn run(&self) {
        match self {
            Self::Unsubscribe(c) => c.run(),
            _ => unimplemented!("not implemented yet"),
        }
    }
}

/// Find and unsubscribe email subscriptions
#[derive(Args, Debug)]
pub struct EmailUnsubscribeArgs {
    // email: String,
}

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

impl EmailUnsubscribeArgs {
    fn run(&self) {
        const API: &[&str] = &["Google", "Microsoft"];
        let api = terminal::choose("Choose API: ", API).expect("error whoile selecting api");
        println!("api chosen: {}", API[api as usize]);

        // let api_config = secrets::ApiConfig::load();
        // gmail::get_credentials(&api_config);
    }
}
