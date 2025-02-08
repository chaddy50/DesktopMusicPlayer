use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Genre {
    id: i64,
    pub name: String,
}

impl Genre {
    pub fn new(id: i64, name: String) -> Self {
        Genre { id, name }
    }
}
