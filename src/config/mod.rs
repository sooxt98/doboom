use std::io::prelude::*;
use std::fs::File;
use toml;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwt {
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Facebook {
    pub app_id: String,
    pub app_secret: String,
    pub redirect_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Twitter {
    pub consumer_key: String,
    pub consumer_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Google {
    pub client_id: String,
    pub app_secret: String,
    pub redirect_uri: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub Jwt: Jwt,
    pub FacebookOauth: Facebook,
    pub TwitterOauth: Twitter,
    pub GoogleOauth: Google,
}

pub fn parse() -> Config {
    dotenv().ok();

    let config_file_path = env::var("CONFIG_FILE")
        .expect("CONFIG_FILE must be set.");

    // Read file path
    let mut file = File::open(config_file_path)
        .expect("Cannot access config file.");

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Cannot read content of config file.");

    let config: Config = toml::from_str(content.as_str())
        .expect("Cannot parse config file.");

    config
}