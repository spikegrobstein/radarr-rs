use reqwest;
extern crate url;
use url::form_urlencoded;

use std::error::Error;

use super::config;
use super::search_result::SearchResult;
use super::status_response::StatusResponse;
use super::health_response::HealthResponse;
use super::root_folder_response::RootFolderResponse;
use super::movie_response::MovieResponse;

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
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("term", term)
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.url_for("movie/lookup", &query_string);
        let body = reqwest::get(&url)?.text()?;
        let results: Vec<SearchResult> = serde_json::from_str(&body)?;

        if results.len() == 0 {
            return Ok(None);
        }

        Ok(Some(results))
    }

    pub fn status(&self) -> Result<StatusResponse, Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.url_for("system/status", &query_string);
        let body = reqwest::get(&url)?.text()?;

        let status = serde_json::from_str(&body)?;
        
        Ok(status)
    }

    pub fn health(&self) -> Result<Vec<HealthResponse>, Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.url_for("health", &query_string);
        let body = reqwest::get(&url)?.text()?;

        let health: Vec<HealthResponse> = serde_json::from_str(&body)?;
        
        Ok(health)
    }

    pub fn root_folder(&self) -> Result<Vec<RootFolderResponse>, Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.url_for("rootfolder", &query_string);
        let body = reqwest::get(&url)?.text()?;

        let root_folder: Vec<RootFolderResponse> = serde_json::from_str(&body)?;
        
        Ok(root_folder)
    }

    pub fn list_movies(&self) -> Result<Vec<MovieResponse>, Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.url_for("movie", &query_string);
        let body = reqwest::get(&url)?.text()?;

        // println!("{}", body);

        let movies: Vec<MovieResponse> = serde_json::from_str(&body)?;

        Ok(movies)
    }

    pub fn movie(&self, id: u32) -> Result<MovieResponse, Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let uri = &format!("movie/{id}", id = id);
        let url = self.url_for(uri, &query_string);
        let body = reqwest::get(&url)?.text()?;

        let movie: MovieResponse = serde_json::from_str(&body)?;

        Ok(movie)
    }

    pub fn url_for(&self, uri: &str, query_string: &str) -> String {
        format!("{}://{}/api/{}?{}",
                &self.config.protocol,
                &self.config.hostname,
                uri,
                query_string)
    }
}
