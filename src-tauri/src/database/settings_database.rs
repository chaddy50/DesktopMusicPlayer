use diesel::{
    prelude::{Insertable, Queryable},
    ExpressionMethods, QueryDsl, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::{database, schema::settings};

#[derive(Serialize, Deserialize, Insertable, Queryable, Clone)]
pub struct Setting {
    key: String,
    pub value: String,
}

const KEY_DIRECTORIES: &str = "directories";

pub fn get_directories() -> Vec<String> {
    let mut database_connection = database::open_database_connection();

    settings::dsl::settings
        .select(settings::value)
        .filter(settings::key.eq(KEY_DIRECTORIES))
        .load::<String>(&mut database_connection)
        .unwrap()
}

pub fn load_settings(app_handle: AppHandle) {
    let mut database_connection = database::open_database_connection();

    let settings = settings::dsl::settings
        .select((settings::key, settings::value))
        .load::<Setting>(&mut database_connection)
        .unwrap();

    app_handle.emit("settings_changed", settings).unwrap();
}

pub fn save_settings(directories: Vec<String>) {
    let mut database_connection = database::open_database_connection();

    diesel::delete(settings::table)
        .filter(settings::key.eq(KEY_DIRECTORIES))
        .execute(&mut database_connection)
        .unwrap();

    for directory in directories {
        let new_setting = Setting {
            key: KEY_DIRECTORIES.to_string(),
            value: directory,
        };
        diesel::insert_into(settings::table)
            .values(new_setting)
            .execute(&mut database_connection)
            .unwrap();
    }
}
