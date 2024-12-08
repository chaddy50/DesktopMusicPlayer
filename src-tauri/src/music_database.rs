use std::{
    fs::{self, DirEntry},
    io::Error,
    path::{Path, PathBuf},
};

use audiotags::{Picture, Tag};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use sqlite::{Connection, State};

const DATABASE_PATH_MUSIC: &str = "music_database.db";

const TABLE_SONGS: &str = "songs";
const TABLE_ALBUMS: &str = "albums";
const TABLE_ALBUM_ARTISTS: &str = "album_artists";
const TABLE_GENRES: &str = "genres";
const COLUMN_ID: &str = "id";
const COLUMN_NAME: &str = "name";
const COLUMN_GENRE: &str = "genre";
const COLUMN_ALBUM_ARTIST: &str = "album_artist";
const COLUMN_ARTIST: &str = "artist";
const COLUMN_ARTWORK_DATA: &str = "artwork_data";
const COLUMN_ALBUM: &str = "album";
const COLUMN_YEAR: &str = "year";
const COLUMN_TRACK_NUMBER: &str = "track_number";
const COLUMN_FILE_PATH: &str = "file_path";

#[derive(Clone, Copy)]
struct TrackToProcess<'a> {
    title: &'a String,
    album: &'a String,
    album_artist: &'a String,
    artist: &'a String,
    genre: &'a String,
    artwork: &'a Picture<'a>,
    file_path: &'a String,
    year: &'a i32,
    track_number: &'a u16,
}

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub name: String,
    album_artist: String,
    artist: String,
    genre: String,
    artwork_source: String,
    pub file_path: String,
    track_number: i64
}

#[derive(Serialize, Deserialize)]
pub struct Album {
    name: String,
    album_artist: String,
    genre: String,
    artwork_source: String,
    year: i64,
    tracks: Vec<Track>,
}

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

    CREATE TABLE IF NOT EXISTS {TABLE_ALBUMS} ({COLUMN_ID} TEXT PRIMARY KEY, {COLUMN_NAME} TEXT, {COLUMN_GENRE} TEXT, {COLUMN_ALBUM_ARTIST} TEXT, {COLUMN_ARTWORK_DATA} TEXT, {COLUMN_YEAR} INT);

    CREATE TABLE IF NOT EXISTS {TABLE_SONGS} ({COLUMN_ID} TEXT PRIMARY KEY, {COLUMN_NAME} TEXT, {COLUMN_GENRE} TEXT, {COLUMN_ALBUM_ARTIST} TEXT, {COLUMN_ALBUM} TEXT, {COLUMN_TRACK_NUMBER} INT, {COLUMN_ARTIST} TEXT, {COLUMN_FILE_PATH} TEXT);
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
    let metadata = Tag::new().read_from_path(&song_file_path);
    match metadata {
        Ok(metadata) => {
            let song = TrackToProcess {
                title: &escape_string_for_sql(metadata.title().unwrap_or_default()),
                album: &escape_string_for_sql(metadata.album().unwrap().title),
                album_artist: &escape_string_for_sql(metadata.album_artist().unwrap_or_default()),
                genre: &escape_string_for_sql(metadata.genre().unwrap_or_default()),
                artwork: &metadata
                    .album_cover()
                    .unwrap_or(Picture::new(&[1], audiotags::MimeType::Png)),
                year: &metadata.year().unwrap_or_default(),
                track_number: &metadata.track_number().unwrap_or_default(),
                artist: &escape_string_for_sql(metadata.artist().unwrap_or_default()),
                file_path: &escape_string_for_sql(song_file_path.as_path().to_str().unwrap_or_default()),
            };

            add_song_to_database(database_connection, song);
            add_album_to_database(database_connection, song);
            add_album_artist_to_database(database_connection, song);
            add_genre_to_database(database_connection, song);
        }
        Err(error) => println!("Error: {}", error),
    }
}

fn add_song_to_database(database_connection: &Connection, song: TrackToProcess) {
    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_SONGS} VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}');
        "#,
        song.file_path,
        song.title,
        song.genre,
        song.album_artist,
        song.album,
        song.track_number,
        song.artist,
        song.file_path,
    );
    let result = database_connection.execute(query);
    match result {
        Ok(_result) => {}
        Err(_error) => {
            println!("Error adding song to database: {}", song.title);
            println!("    {}", song.file_path);
        }
    }
}

fn add_album_to_database(database_connection: &Connection, song: TrackToProcess) {
    let mime_type = song.artwork.mime_type;
    let cover_data = song.artwork.data;
    let cover_data = convert_artwork_data_to_base_64(cover_data);
    let artwork_data = format!("data:image/{:?};base64,{}", mime_type, &cover_data);

    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_ALBUMS} VALUES ('{}', '{}', '{}', '{}', '{}', '{}');
        "#,
        format!("{}{}",song.album,song.album_artist),
        song.album,
        song.genre,
        song.album_artist,
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

fn escape_string_for_sql(str: &str) -> String {
    str.replace('\'', "\'\'")
}

fn add_album_artist_to_database(database_connection: &Connection, song: TrackToProcess) {
    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_ALBUM_ARTISTS} VALUES ('{}', '{}');
        "#,
        song.album_artist,
        song.genre,
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

fn add_genre_to_database(database_connection: &Connection, song: TrackToProcess) {
    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_GENRES} VALUES ('{}');
        "#,
        song.genre
    );
    let result = database_connection.execute(query);
    match result {
        Ok(_result) => {}
        Err(_error) => {
            println!("Error adding genre to database: {}", song.genre);
        }
    }
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
                    escape_string_for_sql(&genre)
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
                    escape_string_for_sql(&album_artist)
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

pub fn get_album_data(album: String) -> Album {
    let database_connection = sqlite::open(DATABASE_PATH_MUSIC);
    match database_connection {
        Ok(database_connection) => {
            let mut statement = database_connection
                .prepare(format!(
                    r#"
                    SELECT * FROM {TABLE_ALBUMS}
                    WHERE {COLUMN_NAME} = '{}'
                    "#,
                    escape_string_for_sql(&album)
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

            let tracks = get_tracks_for_album(&database_connection, album.clone());
            
            Album {
                artwork_source: artwork,
                genre: genre,
                album_artist: album_artist,
                year: year,
                name: album.clone(),
                tracks: tracks,
            }
        }
        Err(error) => {
            println!("Error connecting to database: {}", error);
            Album { name: album, artwork_source: "".to_string(), genre: "".to_string(), album_artist: "".to_string(), year: -1, tracks: Vec::new()}
        }
    }
}

fn get_tracks_for_album(database_connection: &Connection, album: String) -> Vec<Track> {
    let mut tracks = Vec::new();
    let mut statement = database_connection
        .prepare(format!(
            r#"
            SELECT * FROM {TABLE_SONGS}
            WHERE {COLUMN_ALBUM} = '{}'
            ORDER BY {COLUMN_TRACK_NUMBER}
            "#,
            escape_string_for_sql(&album)
        ))
        .unwrap();

    let mut name = "".to_string();
    let mut album_artist = "".to_string();
    let mut artist = "".to_string();
    let mut genre = "".to_string();
    let mut artwork_source = "".to_string();
    let mut file_path = "".to_string();
    let mut track_number = -1;
    while let Ok(State::Row) = statement.next() {
        name = statement.read::<String, _>(COLUMN_NAME).unwrap_or_default();
        album_artist = statement.read::<String, _>(COLUMN_ALBUM_ARTIST).unwrap_or_default();
        artist = statement.read::<String, _>(COLUMN_ARTIST).unwrap_or_default();
        genre = statement.read::<String, _>(COLUMN_GENRE).unwrap_or_default();
        artwork_source = statement.read::<String, _>(COLUMN_ARTWORK_DATA).unwrap_or_default();
        file_path = statement.read::<String, _>(COLUMN_FILE_PATH).unwrap_or_default();
        track_number = statement.read::<i64, _>(COLUMN_TRACK_NUMBER).unwrap_or_default();

        tracks.push(Track { name: name, album_artist: album_artist, artist: artist, genre: genre, artwork_source: artwork_source, file_path: file_path, track_number: track_number });
    }
    tracks
}