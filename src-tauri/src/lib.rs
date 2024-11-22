use sqlite::State;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_genres() -> Vec<String> {
    let mut genres = Vec::new();
    let database_connection = sqlite::open("music_database.db");
    match database_connection {
        Ok(database_connection) => {
            let mut statement = database_connection
                .prepare(
                    r#"
                SELECT * FROM genres 
                ORDER BY name
                "#,
                )
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
                    r#"
                    SELECT * FROM albumArtists 
                    WHERE genre = '{}' AND name <> ""
                    ORDER BY name
                    "#,
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

#[tauri::command]
fn get_albums_for_album_artist(album_artist: String) -> Vec<String> {
    let mut albums = Vec::new();
    let database_connection = sqlite::open("music_database.db");
    match database_connection {
        Ok(database_connection) => {
            let mut statement = database_connection
                .prepare(format!(
                    r#"
                    SELECT * FROM albums
                    WHERE albumArtist = '{}'
                    ORDER BY name
                    "#,
                    album_artist
                ))
                .unwrap();

            while let Ok(State::Row) = statement.next() {
                albums.push(statement.read::<String, _>("name").unwrap());
            }
        }
        Err(error) => {
            println!("Error connecting to database: {}", error);
        }
    }
    albums
}

#[tauri::command]
fn get_tracks_for_album(album: String) -> Vec<String> {
    let mut tracks = Vec::new();
    let database_connection = sqlite::open("music_database.db");
    match database_connection {
        Ok(database_connection) => {
            let mut statement = database_connection
                .prepare(format!(
                    r#"
                    SELECT * FROM songs
                    WHERE album = '{}'
                    ORDER BY name
                    "#,
                    album
                ))
                .unwrap();

            while let Ok(State::Row) = statement.next() {
                tracks.push(statement.read::<String, _>("name").unwrap());
            }
        }
        Err(error) => {
            println!("Error connecting to database: {}", error);
        }
    }
    tracks
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
            get_album_artists_for_genre,
            get_albums_for_album_artist,
            get_tracks_for_album,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
