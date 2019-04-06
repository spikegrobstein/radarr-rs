use serde::{Serialize, Deserialize};

// message: "Indexers unavailable due to failures: Nzb.su"
// type: "warning"
// wikiUrl: "https://github.com/Radarr/Radarr/wiki/Health-checks#indexers-are-unavailable-due-to-failures"

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthResponse {
    pub message: String,

    #[serde(rename = "type")]
    pub log_level: String,

    #[serde(rename = "wikiUrl")]
    pub wiki_url: String,
}
