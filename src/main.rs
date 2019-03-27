use reqwest;
extern crate url;
use url::form_urlencoded;

use std::error::Error;
use std::env;

#[derive(Debug)]
struct RadarrConfig {
    api_token: String,
    hostname: String,
    protocol: String,
}

impl RadarrConfig {
    pub fn new(api_token: String, hostname: String) -> Option<RadarrConfig> {
        Some(RadarrConfig {
            api_token,
            hostname,
            protocol: String::from("http"),
        })
    }

    pub fn new_from_env() -> Option<RadarrConfig> {
        let api_token = env::var("RADARR_API_TOKEN")
            .expect("RADARR_API_TOKEN environment variable must be set");

        let hostname = env::var("RADARR_API_HOSTNAME")
            .unwrap_or(String::from("localhost"));

        let protocol = env::var("RADARR_API_PROTOCOL")
            .unwrap_or(String::from("http"));

        Some(RadarrConfig {
            api_token,
            hostname,
            protocol,
        })
    }
}

fn main() {
    let config = RadarrConfig::new_from_env().unwrap();

    eprintln!("{:?}", config);

    match search(&config, "The Shining") {
        Ok(body) => println!("{}", body),
        Err(error) => panic!("Error: {}", error),
    }
}

fn search(config: &RadarrConfig, title: &str) -> Result<String, Box<dyn Error>> {
    let title_encoded: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("term", title)
        .append_pair("apikey", &config.api_token)
        .finish();

    let url: &str = &format!("{}://{}/api/movie/lookup?{}", config.protocol, config.hostname, title_encoded);
    let body = reqwest::get(url)?.text()?;

    Ok(body)
}
