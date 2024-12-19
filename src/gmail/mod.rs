use crate::secrets::ApiConfig;
use goupil_util::oauth2;
use goupil_util::{self as util, hashmap};
use rand::distributions::{DistString, Distribution};

/// Code challeng generator
///
/// From [the definition of Google](https://developers.google.com/identity/protocols/oauth2/native-app?hl=fr#step1-code-verifier)
struct CodeVerifier;

impl rand::distributions::Distribution<u8> for CodeVerifier {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: u32 = 26 + 26 + 10 + 4;
        const GEN_ASCII_STR_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                abcdefghijklmnopqrstuvwxyz\
                0123456789\
                -_.~";
        let var = rng.next_u32() % RANGE;
        return GEN_ASCII_STR_CHARSET[var as usize];
    }
}

impl rand::distributions::DistString for CodeVerifier {
    fn append_string<R: rand::Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize) {
        unsafe {
            let v = string.as_mut_vec();
            v.extend(self.sample_iter(rng).take(len));
        }
    }
}
impl CodeVerifier {
    pub fn generate() -> String {
        let mut rng = rand::thread_rng();
        Self::sample_string(&CodeVerifier, &mut rng, 128)
    }
    pub fn generate_sha256() -> String {
        // let sha_code = util::sha::sha256(&Self::generate().into_bytes());
        let mut b64encoded =
            util::base64::url_encode(&util::sha::sha256(&Self::generate().into_bytes()));
        b64encoded.pop(); // drop the = fill
        b64encoded
    }
    pub fn to_sha256(input: String) -> String {
        let mut b64encoded = util::base64::url_encode(&util::sha::sha256(&input.into_bytes()));
        b64encoded.pop(); // drop the = fill
        b64encoded
    }
}
pub fn get_credentials(api_config: &ApiConfig) -> oauth2::OAuth2Token {
    let code_verifier = CodeVerifier::generate();
    let code_challenge = CodeVerifier::to_sha256(code_verifier.clone());
    let client_id = &api_config.gmail.client_id;
    let oauth2 = oauth2::OAuth2::new(
        api_config.gmail.client_id.clone(),
        api_config.gmail.client_secret.clone(),
    );
    let params = hashmap! {
        ("response_type", "code".to_string()),
        (
            "scope",
            "https://www.googleapis.com/auth/gmail.readonly".to_string(),
        ),
        ("code_challenge", code_challenge),
        ("code_challenge_method", "S256".to_string()),
    };
    let code = oauth2.get_auth_code("https://accounts.google.com/o/oauth2/v2/auth", params);
    let params = hashmap![("code", code), ("code_verifier", code_verifier),];
    let token = oauth2.get_token("https://oauth2.googleapis.com/token", params);
    log::trace!("tokan obtained: {:?}", token);
    token
}
