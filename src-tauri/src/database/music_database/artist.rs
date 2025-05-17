use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};

use crate::schema::artists;

#[derive(Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = artists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Artist {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = artists)]
pub struct NewArtist {
    pub name: String,
}
