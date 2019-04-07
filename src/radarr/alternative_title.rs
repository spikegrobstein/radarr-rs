use serde::{Serialize, Deserialize};

// "sourceType": "tmdb",
// "movieId": 140,
// "title": "Wanda",
// "sourceId": 31397,
// "votes": 0,
// "voteCount": 0,
// "language": "english",
// "id": 477

#[derive(Serialize, Deserialize, Debug)]
pub struct AlternativeTitle {
    #[serde(rename = "sourceType")]
    pub source_type: String,

    #[serde(rename = "movieId")]
    pub movie_id: u32,

    pub title: String,

    #[serde(rename = "sourceId")]
    pub source_id: u32,

    pub votes: u32,

    #[serde(rename = "voteCount")]
    pub vote_count: u32,

    pub language: String,
    pub id: u32,
}
