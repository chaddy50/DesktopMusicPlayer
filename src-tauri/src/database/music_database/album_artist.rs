use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};

use crate::schema::album_artists;

#[derive(Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = album_artists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AlbumArtist {
    pub id: i32,
    pub name: String,
    pub genre_id: i32,
    pub sort_name: String,
}

impl AlbumArtist {
    pub fn new(id: i32, name: String, genre_id: i32, sort_name: String) -> AlbumArtist {
        AlbumArtist {
            id,
            name,
            genre_id,
            sort_name,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = album_artists)]
pub struct NewAlbumArtist<'a> {
    pub name: &'a String,
    pub genre_id: &'a i32,
    pub sort_name: &'a String,
}
