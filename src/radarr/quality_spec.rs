use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QualitySpec {
    pub id: u32,
    pub name: String,
    pub resolution: u16,
    pub modifier: String,
}
