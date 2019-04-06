use reqwest;
extern crate url;
use url::form_urlencoded;

use std::error::Error;

use super::config;
use super::search_result::SearchResult;

pub struct Client {
    pub config: config::Config,
}

impl Client {
    pub fn new(config: config::Config) -> Client {
        Client {
            config,
        }
    }

    pub fn search(&self, term: &str) -> Result<Option<Vec<SearchResult>>, Box<dyn Error>> {
        let title_encoded: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("term", term)
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.url_for("movie/lookup", &title_encoded);
        let body = reqwest::get(&url)?.text()?;
        let results: Vec<SearchResult> = serde_json::from_str(&body)?;

        if results.len() == 0 {
            return Ok(None);
        }

        Ok(Some(results))
    }

    pub fn url_for(&self, uri: &str, query_string: &str) -> String {
        format!("{}://{}/api/{}?{}",
                &self.config.protocol,
                &self.config.hostname,
                uri,
                query_string)
    }
}
