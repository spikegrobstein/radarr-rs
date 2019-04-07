use serde::{Serialize, Deserialize};

use super::media_info::MediaInfo;
use super::quality::Quality;

#[derive(Serialize, Deserialize, Debug)]
pub struct MovieFile {
    #[serde(rename = "movieId")]
    pub movie_id: u32,

    #[serde(rename = "relativePath")]
    pub relative_path: String,

    pub size: u64,

    #[serde(rename = "dateAdded")]
    pub date_added: String, // datetime

    pub quality: Quality,
    pub edition: String,

    #[serde(rename = "mediaInfo")]
    pub media_info: MediaInfo,
    pub id: u32,
}
