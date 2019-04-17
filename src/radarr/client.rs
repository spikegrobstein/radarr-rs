use reqwest;
extern crate url;
use url::form_urlencoded;

use std::error::Error;

use reqwest::Response;

use super::config;
use super::search_result::SearchResult;
use super::status_response::StatusResponse;
use super::health_response::HealthResponse;
use super::root_folder_response::RootFolderResponse;
use super::movie_response::MovieResponse;
use super::add_movie_payload::AddMoviePayload;
use super::ping_response::PingResponse;
use super::error;

pub struct Client {
    pub config: config::MaterializedConfig,
}

impl Client {
    pub fn new(config: config::Config) -> Result<Client, Box<dyn Error>> {
        let config = config::MaterializedConfig::new_from_config(config)?;

        Ok(Client {
            config,
        })
    }

    pub fn search(&self, term: &str) -> Result<(Response, Option<Vec<SearchResult>>), Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("term", term)
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.api_url_for("movie/lookup", &query_string);
        let mut resp = reqwest::get(&url)?;
        let body = resp.text()?;
        let results: Vec<SearchResult> = serde_json::from_str(&body)?;

        if results.len() == 0 {
            return Ok((resp, None));
        }

        Ok((resp, Some(results)))
    }

    pub fn status(&self) -> Result<(Response, StatusResponse), Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.api_url_for("system/status", &query_string);
        let mut resp = reqwest::get(&url)?;
        let body = resp.text()?;

        let status = serde_json::from_str(&body)?;
        
        Ok((resp, status))
    }

    pub fn health(&self) -> Result<(Response,  Vec<HealthResponse>), Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.api_url_for("health", &query_string);
        let mut resp = reqwest::get(&url)?;
        let body = resp.text()?;

        let health: Vec<HealthResponse> = serde_json::from_str(&body)?;
        
        Ok((resp, health))
    }

    pub fn ping(&self) -> Result<(Response, PingResponse), Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.url_for("signalr/ping", &query_string);
        let mut resp = reqwest::get(&url)?;
        let body = resp.text()?;

        println!("Ping Response: {}", body);

        let ping_response = serde_json::from_str(&body)?;

        Ok((resp, ping_response))
    }

    pub fn root_folder(&self) -> Result<(Response, Vec<RootFolderResponse>), Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.api_url_for("rootfolder", &query_string);
        let mut resp = reqwest::get(&url)?;
        let body = resp.text()?;

        let root_folder: Vec<RootFolderResponse> = serde_json::from_str(&body)?;
        
        Ok((resp, root_folder))
    }

    pub fn list_movies(&self) -> Result<(Response, Vec<MovieResponse>), Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.api_url_for("movie", &query_string);
        let mut resp = reqwest::get(&url)?;
        let body = resp.text()?;

        // println!("{}", body);

        let movies: Vec<MovieResponse> = serde_json::from_str(&body)?;

        Ok((resp, movies))
    }

    pub fn get_movie(&self, id: u32) -> Result<(Response, MovieResponse), Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let uri = &format!("movie/{id}", id = id);
        let url = self.api_url_for(uri, &query_string);
        let mut resp = reqwest::get(&url)?;
        let body = resp.text()?;

        let movie: MovieResponse = serde_json::from_str(&body)?;

        Ok((resp, movie))
    }

    pub fn add_movie(&self, movie: &AddMoviePayload) -> Result<(Response, String), Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.api_url_for("movie", &query_string);
        let client = reqwest::Client::new();

        let payload: String = serde_json::to_string(movie)?;

        // println!("Payload: {}", payload);
        let mut resp = client.post(&url)
            .body(payload)
            .send()?;

        if resp.status().is_success() {
            let body = resp.text()?;
            Ok((resp, body))
        } else {
            let body = resp.text()?;
            // FIXME this should actually percolate up an error
            Err(Box::new(error::UnableToAddMovie::with_msg("Unable to add movie")))
        }
    }

    pub fn delete_movie(&self, movie_id: u32, delete_files: bool) -> Result<(Response, ()), Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();
        
        let uri = format!("movie/{}", movie_id);
        let url = self.api_url_for(&uri, &query_string);
        let client = reqwest::Client::new();

        let mut resp = client.delete(&url).send()?;

        if resp.status().is_success() {
            Ok((resp, ()))
        } else {
            let body = resp.text()?;
            panic!("Failed to delete movie: {}", body);
        }
    }

    pub fn url_for(&self, uri: &str, query_string: &str) -> String {
        format!("{}://{}/{}?{}",
                &self.config.protocol,
                &self.config.hostname,
                uri,
                query_string)
    }

    pub fn api_url_for(&self, uri: &str, query_string: &str) -> String {
        let uri = format!("api/{}", uri);

        self.url_for(&uri, query_string)
    }
}
