use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};

use crate::schema::tracks;

#[derive(Serialize, Deserialize, Clone, Queryable)]
pub struct Track {
    pub name: String,
    album_artist_id: i32,
    album_artist_name: String,
    artist_id: i32,
    artist_name: String,
    genre_id: i32,
    genre_name: String,
    pub file_path: String,
    pub track_number: i32,
    pub disc_number: i32,
    pub duration_in_seconds: i32,
    album_name: String,
}

impl Track {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        album_artist_id: i32,
        album_artist_name: String,
        artist_id: i32,
        artist_name: String,
        genre_id: i32,
        genre_name: String,
        file_path: String,
        track_number: i32,
        disc_number: i32,
        duration_in_seconds: i32,
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

#[derive(Queryable, Selectable)]
#[diesel(table_name = tracks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TrackDatabaseObject {
    pub name: String,
    pub album_artist_id: i32,
    pub artist_id: i32,
    pub genre_id: i32,
    pub file_path: String,
    pub track_number: Option<i32>,
    pub disc_number: Option<i32>,
    pub duration_in_seconds: i32,
}

#[derive(Insertable)]
#[diesel(table_name = tracks)]
pub struct NewTrackDatabaseObject<'a> {
    pub name: String,
    pub genre_id: &'a i32,
    pub album_artist_id: &'a i32,
    pub album_id: &'a i32,
    pub artist_id: &'a i32,
    pub track_number: i32,
    pub disc_number: i32,
    pub file_path: String,
    pub duration_in_seconds: i32,
}
