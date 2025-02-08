use super::track::Track;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Album {
    id: i64,
    pub name: String,
    album_artist_id: i64,
    album_artist_name: String,
    genre_id: i64,
    genre_name: String,
    artwork_source: String,
    year: i64,
    pub tracks: Vec<Track>,
    duration_in_seconds: i64,
}

impl Album {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i64,
        name: String,
        album_artist_id: i64,
        album_artist_name: String,
        genre_id: i64,
        genre_name: String,
        artwork_source: String,
        year: i64,
        tracks: Vec<Track>,
        duration_in_seconds: i64,
    ) -> Self {
        Album {
            id,
            name,
            album_artist_id,
            album_artist_name,
            genre_id,
            genre_name,
            artwork_source,
            year,
            tracks,
            duration_in_seconds,
        }
    }
}
