// pub mod radarr;

use serde::{Serialize, Deserialize};

use crate::radarr::AlternativeTitle;
use crate::radarr::MovieFile;

use super::image::Image;

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    pub title: String,

    #[serde(rename = "secondaryYearSourceId")]
    pub secondary_year_source_id: u32,

    #[serde(rename = "sortTitle")]
    pub sort_title: String,

    #[serde(rename = "alternateTitles")]
    pub alternate_titles: Vec<AlternativeTitle>,

    pub status: String,
    pub overview: String,

    #[serde(rename = "inCinemas")]
    pub in_cinemas: Option<String>,

    pub year: u32,

    pub monitored: bool,

    #[serde(rename = "movieFile")]
    pub movie_file: Option<MovieFile>,

    #[serde(rename = "minimumAvailability")]
    pub minimum_availability: String,

    #[serde(rename = "isAvailable")]
    pub is_available: bool,

    #[serde(rename = "folderName")]
    pub folder_name: String,

    pub runtime: u32,

    #[serde(rename = "tmdbId")]
    pub tmdb_id: u32,

    #[serde(rename = "titleSlug")]
    pub title_slug: String,

    pub genres: Vec<String>,
    pub tags: Vec<String>,
    pub added: String,

    #[serde(rename = "qualityProfileId")]
    pub quality_profile_id: u32,

    pub images: Vec<Image>,
}

