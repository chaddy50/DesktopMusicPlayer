use sqlite::State;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_genres() -> Vec<String> {
    let mut genres = Vec::new();
    let database_connection = sqlite::open("music_database.db");
    match database_connection {
        Ok(database_connection) => {
            let mut statement = database_connection
                .prepare("SELECT * FROM genres ORDER BY name")
                .unwrap();

            while let Ok(State::Row) = statement.next() {
                genres.push(statement.read::<String, _>("name").unwrap());
            }
        }
        Err(error) => {
            println!("Error connecting to database: {}", error);
        }
    }
    genres
}

#[tauri::command]
fn get_album_artists_for_genre(genre: String) -> Vec<String> {
    let mut album_artists = Vec::new();
    let database_connection = sqlite::open("music_database.db");
    match database_connection {
        Ok(database_connection) => {
            let mut statement = database_connection
                .prepare(format!(
                    r#"SELECT * FROM albumArtists WHERE genre = '{}' ORDER BY name"#,
                    genre
                ))
                .unwrap();

            while let Ok(State::Row) = statement.next() {
                album_artists.push(statement.read::<String, _>("name").unwrap());
            }
        }
        Err(error) => {
            println!("Error connecting to database: {}", error);
        }
    }
    album_artists
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Webview,
                ))
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_genres,
            get_album_artists_for_genre
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
