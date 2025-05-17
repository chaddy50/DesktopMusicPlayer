use super::track::Track;
use crate::schema::albums;
use serde::{Deserialize, Serialize};

use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};

#[derive(Serialize, Deserialize)]
pub struct Album {
    id: i32,
    pub name: String,
    album_artist_id: i32,
    album_artist_name: String,
    genre_id: i32,
    genre_name: String,
    artwork_source: String,
    year: i32,
    pub tracks: Vec<Track>,
    duration_in_seconds: i32,
}

impl Album {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i32,
        name: String,
        album_artist_id: i32,
        album_artist_name: String,
        genre_id: i32,
        genre_name: String,
        artwork_source: String,
        year: i32,
        tracks: Vec<Track>,
        duration_in_seconds: i32,
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

#[derive(Queryable, Selectable)]
#[diesel(table_name = albums)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AlbumDatabaseObject {
    pub id: i32,
    pub name: String,
    pub album_artist_id: i32,
    pub genre_id: i32,
    pub artwork_data: Option<String>,
    pub year: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = albums)]
pub struct NewAlbumDatabaseObject<'a> {
    pub name: &'a String,
    pub album_artist_id: &'a i32,
    pub genre_id: &'a i32,
    pub artwork_data: String,
    pub year: i32,
}
