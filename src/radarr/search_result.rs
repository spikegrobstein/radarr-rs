// pub mod radarr;

use serde::{Serialize, Deserialize};

use super::image::Image;

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    pub title: String,

    #[serde(rename = "alternativeTitles")]
    pub alternative_titles: Vec<String>,

    #[serde(rename = "secondaryYearSourceId")]
    pub secondary_year_source_id: u32,

    #[serde(rename = "sortTitle")]
    pub sort_title: String,

    #[serde(rename = "sizeOnDisk")]
    pub size_on_disk: u32,

    pub status: String,
    pub overview: String,

    #[serde(rename = "inCinemas")]
    pub in_cinemas: Option<String>,

    pub downloaded: bool,
    pub year: u32,

    #[serde(rename = "hasFile")]
    pub has_file: bool,

    #[serde(rename = "profileId")]
    pub profile_id: u32,

    #[serde(rename = "pathState")]
    pub path_state: String,

    pub monitored: bool,

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

