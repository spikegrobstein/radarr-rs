use std::env;

use std::error::Error;
use super::error;

const DEFAULT_HOSTNAME: &str = "localhost:7878";
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

    pub fn merge(&mut self, config: Config) {
        if config.api_token.is_some() {
            self.api_token = config.api_token;
        }

        if config.hostname.is_some() {
            self.hostname = config.hostname;
        }

        if config.protocol.is_some() {
            self.protocol = config.protocol;
        }
    }

    pub fn errors(&self) -> Option<error::ConfigNotMaterializeable> {
        let mut fields = vec![];

        if self.api_token.is_none() {
            fields.push(String::from("api_token"));
        }
        if self.hostname.is_none() {
            fields.push(String::from("hostname"));
        }
        if self.protocol.is_none() {
            fields.push(String::from("protocol"));
        }

        if fields.len() == 0 {
            None
        } else {
            Some(error::ConfigNotMaterializeable::with_fields(fields))
        }
    }
}

pub struct MaterializedConfig {
    pub api_token: String,
    pub hostname: String,
    pub protocol: String,
}

impl MaterializedConfig {
    pub fn new_from_config(config: Config) -> Result<MaterializedConfig, Box<dyn Error>> {
        if let Some(error) = config.errors() {
            return Err(Box::new(error));
        }

        Ok(MaterializedConfig {
            api_token: config.api_token.unwrap(),
            hostname: config.hostname.unwrap(),
            protocol: config.protocol.unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn merge_merges_somes() {
        let mut c1 = Config {
            api_token: None,
            hostname: Some(String::from("c1a")),
            protocol: Some(String::from("c1b")),
        };

        let c2 = Config {
            api_token: Some(String::from("c2a")),
            hostname: None,
            protocol: Some(String::from("c2b")),
        };

        c1.merge(c2);

        assert_eq!(&c1.api_token.unwrap(), "c2a");
        assert_eq!(&c1.hostname.unwrap(), "c1a");
        assert_eq!(&c1.protocol.unwrap(), "c2b");
    }
}   
