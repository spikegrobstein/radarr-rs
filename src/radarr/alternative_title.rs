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
    pub title: String,
}
