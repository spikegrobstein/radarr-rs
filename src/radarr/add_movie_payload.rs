use serde::{Serialize, Deserialize};

use super::search_result::SearchResult;
use super::image::Image;
use super::add_options::AddOptions;
// Required:

// title (string)
// qualityProfileId (int)
// titleSlug (string)
// images (array)
// tmdbId (int)
// year (int) release year. Very important needed for the correct path!
// path (string) - full path to the movie on disk
//     or rootFolderPath (string) - full path will be created by combining
//     the rootFolderPath with the movie title
//
// Optional:

// monitored (bool) - whether the movie should be monitored or not.
// addOptions (object) - should contain a searchForMovie (string) key with
// a bool value whether Radarr should search for the movie upon being
// added. For example:

#[derive(Serialize, Deserialize, Debug)]
pub struct AddMoviePayload {
    pub title: String,

    // #[serde(rename = "qualityProfileId")]
    // pub quality_profile_id: u32,

    #[serde(rename = "titleSlug")]
    pub title_slug: String,

    pub images: Vec<Image>,

    #[serde(rename = "tmdbId")]
    pub tmdb_id: u32,
    pub year: u32,
    pub path: Option<String>,

    #[serde(rename = "rootFolderPath")]
    pub root_folder_path: Option<String>,

    pub monitored: Option<bool>,

    #[serde(rename = "addOptions")]
    pub add_options: Option<AddOptions>,
}

impl AddMoviePayload {
    pub fn from_movie_response(movie: &SearchResult) -> Option<AddMoviePayload> {
        if movie.has_file || movie.monitored {
            return None;
        }

        let title = movie.title.to_owned();
        let title_slug = movie.title_slug.to_owned();
        let images = movie.images.to_vec();
        let tmdb_id = movie.tmdb_id;
        let year = movie.year;
        let path = None;
        let root_folder_path = None;
        let monitored = None;
        let add_options = None;

        Some(AddMoviePayload {
            title,
            title_slug,
            images,
            tmdb_id,
            year,
            path,
            root_folder_path,
            monitored,
            add_options,
        })
    }

    pub fn set_search_for_movie(&mut self, value: bool) {
        let search_object = AddOptions {
            search_for_movie: value,
            monitor: "movieOnly".into(),
        };

        self.add_options = Some(search_object);
    }

    pub fn set_monitored(&mut self, value: bool) {
        self.monitored = Some(value);
    }

    pub fn set_path(&mut self, value: &str) {
        self.path = Some(String::from(value));
    }

    pub fn set_root_folder_path(&mut self, value: &str) {
        self.root_folder_path = Some(String::from(value));
    }

    pub fn is_valid(&self) -> bool {
        self.path.is_some() || self.root_folder_path.is_some()
    }
}
