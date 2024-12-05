use crate::util::urlencode::IntoUrlEncoded;
use std::{collections::HashMap, process::Command};

use tiny_http::{Request, Response, Server};

/// Make a mini server for OAuth2 redirection
fn make_oauth_server() -> (u16, Server) {
    let result = (3000..3100)
        .map(|port| (port, Server::http(format!("0.0.0.0:{}", port))))
        .find_map(|(port, res_server)| match res_server {
            Ok(server) => Some(Ok((port, server))),
            Err(e) => Some(Err(e)),
        })
        .expect("An error occured while making a server for oauth.")
        .expect("no port available found between 3000 and 3100");
    result
}

/// Open an URL into the browser.
///
/// Inspired from https://docs.rs/open/latest/open/
fn open_url(url: &str) -> Result<(), &'static str> {
    if cfg!(target_os = "linux") {
        for command in &[
            &["wslview", url] as &[&str],
            &["xdg-open", url],
            &["gio", "open", url],
            &["gnome-open", url],
            &["kde-open", url],
        ] {
            if let Ok(_) = Command::new(command[0]).args(&command[1..]).spawn() {
                return Ok(());
            }
        }
    } else if cfg!(target_os = "windows") {
    } else if cfg!(target_os = "macos") {
    } else {
        println!("unable to open an url in a browser. Please open this url in a browser to connect to google : {}", url);
    }
    Err("unable to open url in a browser")
}

pub fn get_credentials(url: &str, mut params: HashMap<&'static str, String>) {
    let (port, server) = make_oauth_server();
    params.insert("redirect_uri", format!("http://127.0.0.1:{}", port));
    open_url(&format!("{}/{}", url, params.into_url_encoded()));
}
