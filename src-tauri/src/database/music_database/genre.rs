use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};

use crate::schema::genres;

#[derive(Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = genres)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Genre {
    pub id: i32,
    pub name: String,
}

impl Genre {
    pub fn new(id: i32, name: String) -> Self {
        Genre { id, name }
    }
}

#[derive(Insertable)]
#[diesel(table_name = genres)]
pub struct NewGenre {
    pub name: String,
}
