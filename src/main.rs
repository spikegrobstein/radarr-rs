use reqwest;
extern crate url;
use url::form_urlencoded;

use std::error::Error;
use std::env;

use serde::{Serialize, Deserialize};

// extern crate alamo_movies;
use alamo_movies::cinema::Cinema;

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

#[derive(Serialize, Deserialize, Debug)]
struct RadarrSearchResult {
    title: String,
    #[serde(rename = "alternativeTitles")]
    alternative_titles: Vec<String>,
    #[serde(rename = "secondaryYearSourceId")]
    secondary_year_source_id: u32,
    #[serde(rename = "sortTitle")]
    sort_title: String,
    #[serde(rename = "sizeOnDisk")]
    size_on_disk: u32,
    status: String,
    overview: String,
    #[serde(rename = "inCinemas")]
    in_cinemas: Option<String>,
    downloaded: bool,
    year: u32,
    #[serde(rename = "hasFile")]
    has_file: bool,
    #[serde(rename = "profileId")]
    profile_id: u32,
    #[serde(rename = "pathState")]
    path_state: String,
    monitored: bool,
    #[serde(rename = "minimumAvailability")]
    minimum_availability: String,
    #[serde(rename = "isAvailable")]
    is_available: bool,
    #[serde(rename = "folderName")]
    folder_name: String,
    runtime: u32,
    #[serde(rename = "tmdbId")]
    tmdb_id: u32,
    #[serde(rename = "titleSlug")]
    title_slug: String,
    genres: Vec<String>,
    tags: Vec<String>,
    added: String,
    #[serde(rename = "qualityProfileId")]
    quality_profile_id: u32,
}


// how this should work:
// iterate over every cinema from alamo
// filter out all movies of show-type terror tuesday, weird wednesday, video vortext
// search radarr API
// from those results, find best match. best match would be exact match on title or any
// "alternativeTitles"
// add any movies that are not already added.

fn main() {
    let config = RadarrConfig::new_from_env().unwrap();

    let cinema_id = Cinema::to_cinema_id("new-mission").unwrap();
    let body = Cinema::get_calendar_data(&cinema_id).expect("expected thing");
    let (_cinema, films) = Cinema::from_calendar_data(&body).expect("expected thing");

    for film in films {
        if &film.show_type != "Terror Tuesday" {
            continue;
        }

        let title = &film.name;

        match search(&config, title) {
            Ok(Some(results)) => {
                let num_results = results.len();
                println!("Got back {} results for {}", num_results, title);
                if let Some(best) = best_matches(title, results) {
                    println!("Best results {}/{} for {}", best.len(), num_results, title);
                } else {
                    eprintln!("Found no exact matches for {}", title); 
                }
            },
            Ok(None) => eprintln!("Got no results for `{}`.", film.name),
            Err(error) => panic!("Error: {}", error),
        }
    }
}

/// given the results from the radarr api, return an array of best matches
/// best matches are exact matches or exact alternativeNames matches
fn best_matches(term: &str, results: Vec<RadarrSearchResult>) -> Option<Vec<RadarrSearchResult>> {
    let matches: Vec<RadarrSearchResult> = results.into_iter()
        .filter(|result| {
            result.title.to_lowercase() == term.to_lowercase() 
                || result.alternative_titles.iter()
                    .any(|title| title.to_lowercase() == term.to_lowercase())
        })
        .collect();

    if matches.len() == 0 {
        return None;
    }

    Some(matches)
}

fn url_for(uri: &str, config: &RadarrConfig, query_string: &str) -> &str {
    &format!("{}://{}/api/{}?{}", config.protocol, config.hostname, uri, query_string)
}

/// search the radarr api for the given string
fn search(config: &RadarrConfig, title: &str) -> Result<Option<Vec<RadarrSearchResult>>, Box<dyn Error>> {
    let title_encoded: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("term", title)
        .append_pair("apikey", &config.api_token)
        .finish();

    let url: &str = &format!("{}://{}/api/movie/lookup?{}", config.protocol, config.hostname, title_encoded);
    let body = reqwest::get(url)?.text()?;
    let results: Vec<RadarrSearchResult> = serde_json::from_str(&body)?;

    if results.len() == 0 {
        return Ok(None);
    }

    // let results: Vec<RadarrSearchResult> = json.as_array().iter()
        // .map(|result| serde_json::from_str(&result))
        // .collect();

    Ok(Some(results))
}
