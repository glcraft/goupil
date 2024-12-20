use clap::{Args, Subcommand};
use log::info;
use strum::{Display, FromRepr, VariantArray};

use crate::terminal;

#[derive(Args, Debug)]
pub struct EmailArgs {
    #[command(subcommand)]
    pub command: EmailCommands,
}

#[derive(Subcommand, Debug)]
#[non_exhaustive]
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
#[derive(Display, VariantArray, FromRepr, PartialEq)]
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
        let api = terminal::choose_value("Choose API: ", &EmailKind::VARIANTS)
            .expect("error while selecting api");
        info!("api chosen: {}", api);
        match api {
            EmailKind::Gmail => todo!(),
            EmailKind::Microsoft => unimplemented!(),
            EmailKind::Generic => unimplemented!(),
        }

        // let api_config = secrets::ApiConfig::load();
        // gmail::get_credentials(&api_config);
    }
}
