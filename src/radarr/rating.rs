use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Rating {
    pub votes: u32,
    pub value: f32,
}
