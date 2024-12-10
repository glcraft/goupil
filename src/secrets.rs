use serde::Deserialize;

#[derive(Deserialize)]
pub struct Gmail {
    pub client_id: String,
    pub client_secret: String,
}
#[derive(Deserialize)]
pub struct ApiConfig {
    pub gmail: Gmail,
}
impl ApiConfig {
    pub fn load() -> ApiConfig {
        Self::load_with("./secrets.yml")
    }
    pub fn load_with<P: AsRef<std::path::Path>>(path: P) -> ApiConfig {
        let file = std::fs::File::open(path.as_ref()).expect("unable to find the api.yml file");
        serde_yml::from_reader(file).expect("unable to load the api config file")
    }
}
