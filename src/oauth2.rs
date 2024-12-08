use crate::util::{self, urlencode::IntoUrlEncoded};
use core::{panic, str::next_code_point};
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
    println!("Opening {}", url);
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
        todo!();
    } else if cfg!(target_os = "macos") {
        todo!();
    } else {
        println!("Unable to open an url in a browser with your device. Please open this url in a browser to connect to google : {}", url);
    }
    Err("unable to open url in a browser")
}

pub fn get_auth_code(url: &str, mut params: HashMap<&'static str, String>) -> String {
    let (port, server) = make_oauth_server();
    params.insert("redirect_uri", format!("http://127.0.0.1:{}", port));
    open_url(&format!("{}?{}", url, params.into_url_encoded()))
        .expect("unable to open url in a browser");
    let code = match server.recv() {
        Ok(req) => {
            let url = req.url();
            let params = util::urlencode::decode_url_parameters(url)
                .expect("no parameters obtained by oauth2 service");
            if let Some(error) = params.get("error") {
                match error.as_str() {
                    "access_denied" => panic!("The user denied the request."),
                    e => panic!("An error happened while authentifying the user: {}", e),
                }
            }
            params.get("code").expect("\"code\" donc exists").clone()
        }
        Err(e) => panic!("oauth client got an issue: {:?}", e),
    };
    code
}
pub fn get_token(host: &str, mut params: HashMap<&'static str, String>) -> String {
    params.insert("grant_type", "authorization_code".to_string());
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(host)
        .form(&params)
        .send()
        .expect("An error occured while ");
}
