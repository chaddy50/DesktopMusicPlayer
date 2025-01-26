use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Artist {
    pub name: String,
}