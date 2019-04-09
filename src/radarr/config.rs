use std::env;

const DEFAULT_HOSTNAME: &str = "localhost";
const DEFAULT_PROTOCOL: &str = "http";

const ENV_RADARR_API_TOKEN: &str = "RADARR_API_TOKEN";
const ENV_RADARR_API_PROTOCOL: &str = "RADARR_API_PROTOCOL";
const ENV_RADARR_API_HOSTNAME: &str = "RADARR_API_HOSTNAME";

#[derive(Debug)]
pub struct Config {
    pub api_token: Option<String>,
    pub hostname: Option<String>,
    pub protocol: Option<String>,
}

impl Config {
    pub fn new_with_defaults(api_token: String, hostname: Option<String>) -> Config {
        Config {
            api_token: Some(api_token),
            hostname: hostname.or(Some(String::from(DEFAULT_HOSTNAME))),
            protocol: Some(String::from(DEFAULT_PROTOCOL)),
        }
    }

    pub fn new_from_env_with_defaults() -> Config {
        let api_token: Option<String> = match env::var(ENV_RADARR_API_TOKEN) {
            Ok(api_token) => Some(String::from(api_token)),
            Err(_) => None,
        };

        let hostname = env::var(ENV_RADARR_API_HOSTNAME)
            .unwrap_or(String::from(DEFAULT_HOSTNAME));

        let protocol = env::var(ENV_RADARR_API_PROTOCOL)
            .unwrap_or(String::from(DEFAULT_PROTOCOL));

        Config {
            api_token,
            hostname: Some(hostname),
            protocol: Some(protocol),
        }
    }

    pub fn new_from_env() -> Config {
        let api_token = env::var(ENV_RADARR_API_TOKEN).ok();
        let hostname = env::var(ENV_RADARR_API_HOSTNAME).ok();
        let protocol = env::var(ENV_RADARR_API_PROTOCOL).ok();

        Config {
            api_token,
            hostname,
            protocol,
        }
    }

    pub fn can_be_materialized(&self) -> bool {
        self.api_token.is_some()
            && self.hostname.is_some()
            && self.protocol.is_some()
    }
}

pub struct MaterializedConfig {
    pub api_token: String,
    pub hostname: String,
    pub protocol: String,
}

impl MaterializedConfig {
    pub fn new_from_config(config: Config) -> Option<MaterializedConfig> {
        if ! config.can_be_materialized() {
            return None;
        }

        Some(MaterializedConfig {
            api_token: config.api_token.unwrap(),
            hostname: config.hostname.unwrap(),
            protocol: config.protocol.unwrap(),
        })
    }
}
