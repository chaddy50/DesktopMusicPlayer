use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Track {
    pub name: String,
    album_artist_id: i64,
    album_artist_name: String,
    artist_id: i64,
    artist_name: String,
    genre_id: i64,
    genre_name: String,
    pub file_path: String,
    pub track_number: i64,
    pub disc_number: i64,
    pub duration_in_seconds: i64,
    album_name: String,
}

impl Track {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        album_artist_id: i64,
        album_artist_name: String,
        artist_id: i64,
        artist_name: String,
        genre_id: i64,
        genre_name: String,
        file_path: String,
        track_number: i64,
        disc_number: i64,
        duration_in_seconds: i64,
        album_name: String,
    ) -> Track {
        Track {
            name,
            album_artist_id,
            album_artist_name,
            artist_id,
            artist_name,
            genre_id,
            genre_name,
            file_path,
            track_number,
            disc_number,
            duration_in_seconds,
            album_name,
        }
    }
}
