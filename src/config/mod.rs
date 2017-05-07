use std::io::prelude::*;
use std::fs::File;
use toml;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Server {
    /// Including the port.
    pub addr: String,
}

#[derive(Debug, Deserialize)]
pub struct Postgres {
    pub addr: String,
}

#[derive(Debug, Deserialize)]
pub struct Jwt {
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub postgres: Postgres,
    pub jwt: Jwt,
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