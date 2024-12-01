use std::{
    fs::{self, DirEntry},
    io::Error,
    path::Path,
    path::PathBuf,
};

use audiotags::{Picture, Tag};
use base64::{engine::general_purpose, Engine as _};
use sqlite::{Connection, State};
use tauri::ipc::{private::ResponseKind, IpcResponse};

const DATABASE_PATH_MUSIC: &str = "music_database.db";

const TABLE_SONGS: &str = "songs";
const TABLE_ALBUMS: &str = "albums";
const TABLE_ALBUM_ARTISTS: &str = "album_artists";
const TABLE_GENRES: &str = "genres";
const COLUMN_NAME: &str = "name";
const COLUMN_GENRE: &str = "genre";
const COLUMN_ALBUM_ARTIST: &str = "album_artist";
const COLUMN_ARTWORK_DATA: &str = "artwork_data";
const COLUMN_ALBUM: &str = "album";
const COLUMN_YEAR: &str = "year";
const COLUMN_TRACK_NUMBER: &str = "track_number";

pub fn build_music_database() {
    if Path::new(DATABASE_PATH_MUSIC).exists() {
        return;
    }

    let database_connection = sqlite::open(DATABASE_PATH_MUSIC);
    match database_connection {
        Ok(database_connection) => {
            create_database_tables(&database_connection);

            retrieve_audio_files_from_directory(
                &database_connection,
                "/home/nathan/Music/Video Game",
            );
            retrieve_audio_files_from_directory(&database_connection, "/home/nathan/Music/Rock");
            retrieve_audio_files_from_directory(&database_connection, "/home/nathan/Music/Jazz");
            retrieve_audio_files_from_directory(
                &database_connection,
                "/home/nathan/Music/Classic Rock",
            );
            retrieve_audio_files_from_directory(&database_connection, "/home/nathan/Music/Ambient");
            retrieve_audio_files_from_directory(
                &database_connection,
                "/home/nathan/Music/Electronic",
            );
        }
        Err(error) => {
            println!("Error connecting to database: {}", error);
        }
    }
}

fn create_database_tables(database_connection: &Connection) {
    let query = format!("
    CREATE TABLE IF NOT EXISTS {TABLE_GENRES} ({COLUMN_NAME} TEXT PRIMARY KEY);

    CREATE TABLE IF NOT EXISTS {TABLE_ALBUM_ARTISTS} ({COLUMN_NAME} TEXT PRIMARY KEY, {COLUMN_GENRE} TEXT);

    CREATE TABLE IF NOT EXISTS {TABLE_ALBUMS} ({COLUMN_NAME} TEXT PRIMARY KEY, {COLUMN_GENRE} TEXT, {COLUMN_ALBUM_ARTIST} TEXT, {COLUMN_ARTWORK_DATA} TEXT, {COLUMN_YEAR} INT);

    CREATE TABLE IF NOT EXISTS {TABLE_SONGS} ({COLUMN_NAME} TEXT PRIMARY KEY, {COLUMN_GENRE} TEXT, {COLUMN_ALBUM_ARTIST} TEXT, {COLUMN_ALBUM} TEXT, {COLUMN_TRACK_NUMBER} INT);
    ");
    database_connection.execute(query).unwrap();
}

fn retrieve_audio_files_from_directory(database_connection: &Connection, path: &str) {
    let paths = fs::read_dir(path);
    match paths {
        Ok(paths) => {
            for path in paths {
                match path {
                    Ok(directory) => {
                        read_directory(database_connection, directory);
                    }
                    Err(error) => println!("There was an error reading path: {}", error),
                }
            }
        }
        Err(error) => println!("There was an error retrieving audio files: {}", error),
    }
}

fn read_directory(database_connection: &Connection, directory: DirEntry) {
    if directory.file_type().unwrap().is_dir() {
        let directory_path = directory.path();
        let directory_path = directory_path.to_str();
        match directory_path {
            Some(directory_path) => {
                read_files_in_directory(database_connection, directory_path);

                // After reading the files, see if we need to traverse further down the file tree
                retrieve_audio_files_from_directory(database_connection, directory_path);
            }
            None => println!("There was an error"),
        }
    }
}

fn read_files_in_directory(database_connection: &Connection, directory_path: &str) {
    let files = fs::read_dir(directory_path);

    match files {
        Ok(files) => {
            for file in files {
                read_file(database_connection, file);
            }
        }
        Err(error) => println!("There was an error reading files in a directory: {}", error),
    }
}

fn read_file<'a>(database_connection: &Connection, file: Result<DirEntry, Error>) {
    match file {
        Ok(file) => {
            let file_path = file.path();
            let file_name = file.file_name();
            let file_name = file_name.to_str();
            match file_name {
                Some(file_name) => {
                    if file_name.ends_with(".flac") {
                        process_song(database_connection, file_path);
                    }
                }
                None => println!("No file name"),
            }
        }
        Err(error) => println!("There was an error reading a file: {}", error),
    }
}

fn process_song(database_connection: &Connection, song_file_path: PathBuf) {
    let metadata = Tag::new().read_from_path(song_file_path);
    match metadata {
        Ok(metadata) => {
            let song = Track {
                title: metadata.title().unwrap_or_default(),
                album: metadata.album().unwrap().title,
                album_artist: metadata.album_artist().unwrap_or_default(),
                genre: metadata.genre().unwrap_or_default(),
                artwork: &metadata
                    .album_cover()
                    .unwrap_or(Picture::new(&[1], audiotags::MimeType::Png)),
                year: &metadata.year().unwrap_or_default(),
                track_number: &metadata.track_number().unwrap_or_default(),
            };

            add_song_to_database(database_connection, song);
            add_album_to_database(database_connection, song);
            add_album_artist_to_database(database_connection, song);
            add_genre_to_database(database_connection, song);
        }
        Err(error) => println!("Error: {}", error),
    }
}

fn add_song_to_database(database_connection: &Connection, song: Track) {
    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_SONGS} VALUES ('{}', '{}', '{}', '{}', '{}');
        "#,
        escape_apostrophe(song.title),
        escape_apostrophe(song.genre),
        escape_apostrophe(song.album_artist),
        escape_apostrophe(song.album),
        song.track_number
    );
    let result = database_connection.execute(query);
    match result {
        Ok(_result) => {}
        Err(_error) => {
            println!("Error adding song to database: {}", song.title);
        }
    }
}

fn add_album_to_database(database_connection: &Connection, song: Track) {
    let mime_type = song.artwork.mime_type;
    let cover_data = song.artwork.data;
    let cover_data = convert_artwork_data_to_base_64(cover_data);
    let artwork_data = format!("data:image/{:?};base64,{}", mime_type, &cover_data);

    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_ALBUMS} VALUES ('{}', '{}', '{}', '{}', '{}');
        "#,
        escape_apostrophe(song.album),
        escape_apostrophe(song.genre),
        escape_apostrophe(song.album_artist),
        artwork_data,
        song.year,
    );

    let result = database_connection.execute(query);
    match result {
        Ok(_result) => {}
        Err(_error) => {
            println!("Error adding album to database: {}", song.album);
        }
    }
}

fn convert_artwork_data_to_base_64(artwork_data: &[u8]) -> String {
    general_purpose::STANDARD.encode(artwork_data)
}

fn escape_apostrophe(str: &str) -> String {
    str.replace('\'', "\'\'")
}

fn add_album_artist_to_database(database_connection: &Connection, song: Track) {
    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_ALBUM_ARTISTS} VALUES ('{}', '{}');
        "#,
        escape_apostrophe(song.album_artist),
        escape_apostrophe(song.genre),
    );
    let result = database_connection.execute(query);
    match result {
        Ok(_result) => {}
        Err(_error) => {
            println!(
                "Error adding album artist to database: {}",
                song.album_artist
            );
        }
    }
}

fn add_genre_to_database(database_connection: &Connection, song: Track) {
    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_GENRES} VALUES ('{}');
        "#,
        escape_apostrophe(song.genre)
    );
    let result = database_connection.execute(query);
    match result {
        Ok(_result) => {}
        Err(_error) => {
            println!("Error adding genre to database: {}", song.genre);
        }
    }
}

#[derive(Clone, Copy)]
struct Track<'a> {
    title: &'a str,
    album: &'a str,
    album_artist: &'a str,
    genre: &'a str,
    artwork: &'a Picture<'a>,
    year: &'a i32,
    track_number: &'a u16,
}

#[derive(serde::Serialize)]
pub struct Album {
    name: String,
    album_artist: String,
    genre: String,
    artwork_source: String,
    year: i64,
    tracks: Vec<String>,
}

pub fn get_genres() -> Vec<String> {
    let mut genres = Vec::new();
    let database_connection = sqlite::open(DATABASE_PATH_MUSIC);
    match database_connection {
        Ok(database_connection) => {
            let mut statement = database_connection
                .prepare(format!(
                    r#"
                SELECT * FROM {TABLE_GENRES} 
                ORDER BY {COLUMN_NAME}
                "#
                ))
                .unwrap();

            while let Ok(State::Row) = statement.next() {
                genres.push(statement.read::<String, _>(COLUMN_NAME).unwrap());
            }
        }
        Err(error) => {
            println!("Error connecting to database: {}", error);
        }
    }
    genres
}

pub fn get_album_artists_for_genre(genre: String) -> Vec<String> {
    let mut album_artists = Vec::new();
    let database_connection = sqlite::open(DATABASE_PATH_MUSIC);
    match database_connection {
        Ok(database_connection) => {
            let mut statement = database_connection
                .prepare(format!(
                    r#"
                    SELECT * FROM {TABLE_ALBUM_ARTISTS} 
                    WHERE genre = '{}' AND {COLUMN_NAME} <> ""
                    ORDER BY {COLUMN_NAME}
                    "#,
                    escape_apostrophe(&genre)
                ))
                .unwrap();

            while let Ok(State::Row) = statement.next() {
                album_artists.push(statement.read::<String, _>(COLUMN_NAME).unwrap());
            }
        }
        Err(error) => {
            println!("Error connecting to database: {}", error);
        }
    }
    album_artists
}

pub fn get_albums_for_album_artist(album_artist: String) -> Vec<String> {
    let mut albums = Vec::new();
    let database_connection = sqlite::open(DATABASE_PATH_MUSIC);
    match database_connection {
        Ok(database_connection) => {
            let mut statement = database_connection
                .prepare(format!(
                    r#"
                    SELECT * FROM {TABLE_ALBUMS}
                    WHERE {COLUMN_ALBUM_ARTIST} = '{}'
                    ORDER BY {COLUMN_YEAR}
                    "#,
                    escape_apostrophe(&album_artist)
                ))
                .unwrap();

            while let Ok(State::Row) = statement.next() {
                albums.push(statement.read::<String, _>(COLUMN_NAME).unwrap());
            }
        }
        Err(error) => {
            println!("Error connecting to database: {}", error);
        }
    }
    albums
}

pub fn get_tracks_for_album(album: String) -> Vec<String> {
    let mut tracks = Vec::new();
    let database_connection = sqlite::open(DATABASE_PATH_MUSIC);
    match database_connection {
        Ok(database_connection) => {
            let mut statement = database_connection
                .prepare(format!(
                    r#"
                    SELECT * FROM {TABLE_SONGS}
                    WHERE {COLUMN_ALBUM} = '{}'
                    ORDER BY {COLUMN_TRACK_NUMBER}
                    "#,
                    escape_apostrophe(&album)
                ))
                .unwrap();

            while let Ok(State::Row) = statement.next() {
                tracks.push(statement.read::<String, _>(COLUMN_NAME).unwrap());
            }
        }
        Err(error) => {
            println!("Error connecting to database: {}", error);
        }
    }
    tracks
}

pub fn get_artwork_for_album(album: String) -> String {
    let mut artwork_source = String::new();
    let database_connection = sqlite::open(DATABASE_PATH_MUSIC);
    match database_connection {
        Ok(database_connection) => {
            let mut statement = database_connection
                .prepare(format!(
                    r#"
                    SELECT {COLUMN_ARTWORK_DATA} FROM {TABLE_ALBUMS}
                    WHERE {COLUMN_NAME} = '{}'
                    "#,
                    escape_apostrophe(&album)
                ))
                .unwrap();

            while let Ok(State::Row) = statement.next() {
                artwork_source = statement.read::<String, _>(COLUMN_ARTWORK_DATA).unwrap();
            }
        }
        Err(error) => {
            println!("Error connecting to database: {}", error);
        }
    }
    artwork_source
}

pub fn get_album_data(album: String) -> Album {
    let album_data: Album;
    let database_connection = sqlite::open(DATABASE_PATH_MUSIC);
    match database_connection {
        Ok(database_connection) => {
            let mut statement = database_connection
                .prepare(format!(
                    r#"
                    SELECT * FROM {TABLE_ALBUMS}
                    WHERE {COLUMN_NAME} = '{}'
                    "#,
                    escape_apostrophe(&album)
                ))
                .unwrap();

            let mut artwork = "".to_string();
            let mut genre = "".to_string();
            let mut album_artist = "".to_string();
            let mut year = -1;
            while let Ok(State::Row) = statement.next() {
                artwork = statement.read::<String, _>(COLUMN_ARTWORK_DATA).unwrap();
                genre = statement.read::<String, _>(COLUMN_GENRE).unwrap();
                album_artist = statement.read::<String, _>(COLUMN_ALBUM_ARTIST).unwrap();
                year = statement.read::<i64, _>(COLUMN_YEAR).unwrap();
            }

            let tracks = get_tracks_for_album(album.clone());
            
            album_data = Album {
                artwork_source: artwork,
                genre: genre,
                album_artist: album_artist,
                year: year,
                name: album.clone(),
                tracks: tracks,
            };

            album_data
        }
        Err(error) => {
            println!("Error connecting to database: {}", error);
            Album { name: album, artwork_source: "".to_string(), genre: "".to_string(), album_artist: "".to_string(), year: -1, tracks: Vec::new()}
        }
    }
}
