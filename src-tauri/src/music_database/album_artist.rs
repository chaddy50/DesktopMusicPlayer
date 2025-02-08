use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AlbumArtist {
    id: i64,
    pub name: String,
}

impl AlbumArtist {
    pub fn new(id: i64, name: String) -> AlbumArtist {
        AlbumArtist { id, name }
    }
}
