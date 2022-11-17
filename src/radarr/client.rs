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
use super::add_movie_payload::AddMoviePayload;
use super::response::Response;
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

    pub fn search(&self, term: &str) -> Result<Response<Vec<SearchResult>>, Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("term", term)
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.api_url_for("movie/lookup", &query_string);
        let mut resp = reqwest::get(&url)?;
        let body = resp.text()?;
        let results: Vec<SearchResult> = serde_json::from_str(&body)?;

        Ok(Response::new(resp, results))
    }

    pub fn status(&self) -> Result<Response<StatusResponse>, Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.api_url_for("system/status", &query_string);
        let mut resp = reqwest::get(&url)?;
        let body = resp.text()?;

        let status = serde_json::from_str(&body)?;
        
        Ok(Response::new(resp, status))
    }

    pub fn health(&self) -> Result<Response<Vec<HealthResponse>>, Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.api_url_for("health", &query_string);
        let mut resp = reqwest::get(&url)?;
        let body = resp.text()?;

        let health: Vec<HealthResponse> = serde_json::from_str(&body)?;
        
        Ok(Response::new(resp, health))
    }

    pub fn root_folder(&self) -> Result<Response<Vec<RootFolderResponse>>, Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.api_url_for("rootfolder", &query_string);
        let mut resp = reqwest::get(&url)?;
        let body = resp.text()?;

        let root_folder: Vec<RootFolderResponse> = serde_json::from_str(&body)?;

        Ok(Response::new(resp, root_folder))
    }

    pub fn list_movies(&self) -> Result<Response<Vec<MovieResponse>>, Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.api_url_for("movie", &query_string);
        let mut resp = reqwest::get(&url)?;
        let body = resp.text()?;

        // println!("{}", body);

        let movies: Vec<MovieResponse> = serde_json::from_str(&body)?;

        Ok(Response::new(resp, movies))
    }

    pub fn get_movie(&self, id: u32) -> Result<Response<MovieResponse>, Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let uri = &format!("movie/{id}", id = id);
        let url = self.api_url_for(uri, &query_string);
        let mut resp = reqwest::get(&url)?;
        let body = resp.text()?;

        let movie: MovieResponse = serde_json::from_str(&body)?;

        Ok(Response::new(resp, movie))
    }

    pub fn add_movie(&self, movie: &AddMoviePayload) -> Result<Response<String>, Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();

        let url = self.api_url_for("movie", &query_string);
        let client = reqwest::Client::new();

        let payload: String = serde_json::to_string(movie)?;

        // println!("Payload: {}", payload);
        let mut resp = client.post(&url)
            .body(payload)
            .header("content-type", "application/json")
            .send()?;

        if resp.status().is_success() {
            let body = resp.text()?;
            Ok(Response::new(resp, body))
        } else {
            let body = resp.text()?;
            eprintln!("[{}] error body: {body}", resp.status());
            // FIXME this should actually percolate up an error
            Err(Box::new(error::UnableToAddMovie::with_msg("Unable to add movie")))
        }
    }

    pub fn delete_movie(&self, movie_id: u32, delete_files: bool) -> Result<Response<()>, Box<dyn Error>> {
        let query_string: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("apikey", &self.config.api_token)
            .finish();
        
        let uri = format!("movie/{}", movie_id);
        let url = self.api_url_for(&uri, &query_string);
        let client = reqwest::Client::new();

        let mut resp = client.delete(&url).send()?;

        if resp.status().is_success() {
            Ok(Response::new(resp, ()))
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
        let uri = format!("api/v3/{}", uri);

        self.url_for(&uri, query_string)
    }
}
