use serde::{Serialize, Deserialize};

// freeSpace: 1245057187328
// id: 1
// path: "/storage/Movies/"
// totalSpace: 46891755717632

#[derive(Serialize, Deserialize, Debug)]
pub struct RootFolderResponse {
    #[serde(rename = "freeSpace")]
    pub free_space: u64,

    pub id: u32,
    pub path: String,

    #[serde(rename = "totalSpace")]
    pub total_space: u64,
}
