use serde::{Serialize, Deserialize};

use super::rating::Rating;
use super::image::Image;
use super::alternative_title::AlternativeTitle;
use super::movie_file::MovieFile;

// {
  // "title": "Assassin's Creed",
  // "sortTitle": "assassins creed",
  // "sizeOnDisk": 0,
  // "status": "released",
  // "overview": "Lynch discovers he is a descendant of the secret Assassins society through unlocked genetic memories that allow him to relive the adventures of his ancestor, Aguilar, in 15th Century Spain. After gaining incredible knowledge and skills he’s poised to take on the oppressive Knights Templar in the present day.",
  // "inCinemas": "2016-12-21T00:00:00Z",
  // "images": [
    // {
      // "coverType": "poster",
      // "url": "/radarr/MediaCover/1/poster.jpg?lastWrite=636200219330000000"
    // },
    // {
      // "coverType": "banner",
      // "url": "/radarr/MediaCover/1/banner.jpg?lastWrite=636200219340000000"
    // }
  // ],
  // "website": "https://www.ubisoft.com/en-US/",
  // "downloaded": false,
  // "year": 2016,
  // "hasFile": false,
  // "youTubeTrailerId": "pgALJgMjXN4",
  // "studio": "20th Century Fox",
  // "path": "/path/to/Assassin's Creed (2016)",
  // "profileId": 6,
  // "monitored": true,
  // "minimumAvailability": "preDb",
  // "runtime": 115,
  // "lastInfoSync": "2017-01-23T22:05:32.365337Z",
  // "cleanTitle": "assassinscreed",
  // "imdbId": "tt2094766",
  // "tmdbId": 121856,
  // "titleSlug": "assassins-creed-121856",
  // "genres": [
    // "Action",
    // "Adventure",
    // "Fantasy",
    // "Science Fiction"
  // ],
  // "tags": [],
  // "added": "2017-01-14T20:18:52.938244Z",
  // "ratings": {
    // "votes": 711,
    // "value": 5.2
  // },
  // "alternativeTitles": [
    // "Assassin's Creed: The IMAX Experience"
  // ],
  // "qualityProfileId": 6,
  // "id": 1
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct MovieResponse {
    pub title: String,

    #[serde(rename = "sortTitle")]
    pub sort_title: String,

    #[serde(rename = "sizeOnDisk")]
    pub size_on_disk: u64,

    pub status: String,
    pub overview: Option<String>,

    #[serde(rename = "inCinemas")]
    pub in_cinemas: Option<String>, // datetime

    pub images: Vec<Image>,
    pub website: Option<String>,
    pub downloaded: bool,
    pub year: u32,

    #[serde(rename = "hasFile")]
    pub has_file: bool,

    #[serde(rename = "youTubeTrailerId")]
    pub youtube_trailer_id: Option<String>,

    pub studio: Option<String>,
    pub path: String,

    #[serde(rename = "profileId")]
    pub profile_id: u32,

    pub monitored: bool,

    #[serde(rename = "minimumAvailability")]
    pub minimum_availability: String,

    pub runtime: u32,

    #[serde(rename = "lastInfoSync")]
    pub last_info_sync: Option<String>, // datetime

    #[serde(rename = "cleanTitle")]
    pub clean_title: String,

    #[serde(rename = "imdbId")]
    pub imdb_id: Option<String>,

    #[serde(rename = "tmdbId")]
    pub tmdb_id: Option<u32>,

    #[serde(rename = "titleSlug")]
    pub title_slug: String,

    pub genres: Vec<String>,
    pub tags: Vec<String>,
    pub added: String, // datetime

    #[serde(rename = "alternativeTitles")]
    pub alternative_titles: Vec<AlternativeTitle>,

    #[serde(rename = "qualityProfileId")]
    pub quality_profile_id: u32,

    pub id: u32,

    pub edition: Option<String>,

    #[serde(rename = "movieFile")]
    pub movie_file: Option<MovieFile>,
}
