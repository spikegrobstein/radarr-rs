use std::env;

#[derive(Debug)]
pub struct Config {
    pub api_token: String,
    pub hostname: String,
    pub protocol: String,
}

impl Config {
    pub fn new(api_token: String, hostname: String) -> Option<Config> {
        Some(Config {
            api_token,
            hostname,
            protocol: String::from("http"),
        })
    }

    pub fn new_from_env() -> Option<Config> {
        let api_token = env::var("RADARR_API_TOKEN")
            .expect("RADARR_API_TOKEN environment variable must be set");

        let hostname = env::var("RADARR_API_HOSTNAME")
            .unwrap_or(String::from("localhost"));

        let protocol = env::var("RADARR_API_PROTOCOL")
            .unwrap_or(String::from("http"));

        Some(Config {
            api_token,
            hostname,
            protocol,
        })
    }
}


