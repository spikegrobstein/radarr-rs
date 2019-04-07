use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Revision {
    pub version: u32,
    pub real: u32,
}
