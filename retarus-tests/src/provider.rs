
use config::{ConfigError, Config, File};
use retarus::general::{document::Document, creds::Credentials};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub username: String,
    pub password: String,
    pub endpoint: String
}
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("assets/settings.toml"))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}


pub fn provide_settings() -> Settings {
    Settings::new().unwrap()
}


pub fn provide_test_file() -> Document {
    let data = std::fs::read("assets/testPdf.pdf").unwrap();
    let docs = Document::new("test.pdf".to_string(), data, None);
    return docs;
}



pub fn provide_test_credentials() -> Credentials {
    let s = provide_settings();
    Credentials::new(s.username.as_str(), s.password.as_str())
}