use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddOptions {
    #[serde(rename = "searchForMovie")]
    pub search_for_movie: bool,

    pub monitor: String,
}
