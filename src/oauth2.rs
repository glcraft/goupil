use crate::{
    hashmap,
    util::{self, urlencode::IntoUrlEncoded},
};
use core::panic;
use std::{collections::HashMap, process::Command, str::FromStr};

use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tiny_http::{Header, Response, Server, StatusCode};

/// Make a mini server for OAuth2 redirection
fn make_oauth_server() -> (u16, Server) {
    const PORTS: std::ops::Range<u16> = 3000..3100;
    log::info!(
        "Trying open a server with port between {} and {}",
        PORTS.start,
        PORTS.end
    );
    let result = PORTS
        .map(|port| (port, Server::http(format!("0.0.0.0:{}", port))))
        .find_map(|(port, res_server)| match res_server {
            Ok(server) => {
                log::info!("Server 127.0.0.1:{} opened", port);
                Some(Ok((port, server)))
            }
            Err(e) if e.to_string().contains("os error 98") => {
                log::trace!("Port {} already in use", port);
                None
            }
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
    log::info!("Openingan an url in a browser: {}", url);
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
        log::warn!("Unable to open an url in a browser with your device. Please open this url in a browser to connect : {}", url);
    }
    Err("unable to open url in a browser")
}

struct OAuth2ClientId {
    client_id: String,
    client_secret: String,
}
pub struct OAuth2 {
    server: tiny_http::Server,
    server_port: u16,
    id: OAuth2ClientId,
}
impl OAuth2 {
    pub fn new(client_id: String, client_secret: String) -> Self {
        let (port, server) = make_oauth_server();
        Self {
            server,
            server_port: port,
            id: OAuth2ClientId {
                client_id,
                client_secret,
            },
        }
    }
    pub fn get_auth_code(&self, url: &str, params: HashMap<&'static str, String>) -> String {
        log::trace!("get_auth_cod(url = \"{}\", params = {:?})", url, params);
        let params = hashmap![
            params,
            ("redirect_uri", self.redirect_uri()),
            ("client_id", self.id.client_id.clone()),
        ];
        open_url(&format!("{}?{}", url, params.into_url_encoded()))
            .expect("unable to open url in a browser");
        let code = match self.server.recv() {
            Ok(req) => {
                log::info!("server: request intercepted");
                let code = {
                    let params = util::urlencode::decode_url_parameters(req.url())
                        .expect("no parameters obtained by oauth2 service");
                    if let Some(error) = params.get("error") {
                        match error.as_str() {
                            "access_denied" => panic!("The user denied the request."),
                            e => panic!("An error happened while authentifying the user: {}", e),
                        }
                    }
                    params.get("code").expect("\"code\" donc exists").clone()
                };
                log::trace!("code found: {}", code);
                const HTML_OAUTH2_RESPONSE: &str = include_str!("oauth2_response.html");
                req.respond(Response::new(
                    StatusCode(200),
                    unsafe { vec![Header::from_str("Content-Type: text/html").unwrap_unchecked()] },
                    HTML_OAUTH2_RESPONSE.as_bytes(),
                    None,
                    None,
                ))
                .expect("unable to respond to request");
                code
            }
            Err(e) => panic!("oauth client got an issue: {:?}", e),
        };
        code
    }

    pub fn get_token(&self, host: &str, params: HashMap<&'static str, String>) -> OAuth2Token {
        log::trace!("get_token(host = {}, params = {:?})", host, params);
        let params = hashmap![
            params,
            ("grant_type", "authorization_code".to_string()),
            ("redirect_uri", self.redirect_uri()),
            ("client_id", self.id.client_id.clone()),
            ("client_secret", self.id.client_secret.clone()),
        ];
        Self::post_request::<OAuth2TokenResponse>(host, params).into()
    }
    #[inline]
    fn redirect_uri(&self) -> String {
        format!("http://127.0.0.1:{}", self.server_port)
    }

    pub fn refresh_token(&self, url: &str, token: OAuth2Token) -> Option<OAuth2Token> {
        let OAuth2Token {
            access_token: _,
            refresh_token: Some(refresh_token),
            expiration_date: Some(expiration_date),
        } = token
        else {
            return None;
        };
        let params = hashmap![
            ("client_id", self.id.client_id.clone()),
            ("client_secret", self.id.client_secret.clone()),
            ("grant_type", "refresh_token".to_string()),
            ("refresh_token", refresh_token)
        ];
        Some(Self::post_request::<OAuth2TokenResponse>(url, params).into())
    }
    fn post_request<T: DeserializeOwned>(url: &str, params: HashMap<&'static str, String>) -> T {
        let client = reqwest::blocking::Client::new();
        let resp = client
            .post(url)
            .form(&params)
            .send()
            .expect("An error occured during token exchange")
            .text()
            .expect("oauth token: unable to parse text");
        log::trace!("json received: {}", resp);
        let resp_json = serde_json::from_str::<T>(&resp).expect("oauth token malformed");
        resp_json
    }
}

#[derive(Deserialize)]
struct OAuth2TokenResponse {
    access_token: String,
    expires_in: Option<i64>,
    // id_token: Option<String>,
    refresh_token: Option<String>,
    // scope: String,
    // token_type: Stvring,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OAuth2Token {
    pub access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<DateTime<Utc>>,
}

impl From<OAuth2TokenResponse> for OAuth2Token {
    fn from(value: OAuth2TokenResponse) -> Self {
        Self {
            access_token: value.access_token,
            refresh_token: value.refresh_token,
            expiration_date: value
                .expires_in
                .map(|v| Utc::now() + chrono::Duration::seconds(v)),
        }
    }
}
