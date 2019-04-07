use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    #[serde(rename = "coverType")]
    pub cover_type: String,

    pub url: String,
}

