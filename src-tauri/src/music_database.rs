use std::path::PathBuf;

use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use sqlite::{Connection, State, Statement};
use audiotags::{Picture, Tag};

pub const DATABASE_PATH_MUSIC: &str = "music_database.db";

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
const COLUMN_DURATION: &str = "duration";
const COLUMN_ALBUM_ARTIST_SORT_NAME: &str = "album_artist_sort_name";
const COLUMN_DISC_NUMBER: &str = "disc_number";

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct TrackToProcess<'a> {
    title: &'a String,
    album: &'a String,
    album_artist: &'a String,
    artist: &'a String,
    genre: &'a String,
    artwork: &'a Picture<'a>,
    file_path: &'a String,
    year: &'a i32,
    track_number: &'a u16,
    duration: &'a f64,
    disc_number: &'a u16,
}

#[derive(Serialize, Deserialize)]
pub struct NowPlayingQueue {
    pub now_playing_tracks: Vec<Track>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Track {
    pub name: String,
    album_artist: String,
    artist: String,
    genre: String,
    artwork_source: String,
    pub file_path: String,
    track_number: i64,
    disc_number: i64,
    duration_in_seconds: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Album {
    pub name: String,
    album_artist: String,
    genre: String,
    artwork_source: String,
    year: i64,
    pub tracks: Vec<Track>,
    duration_in_seconds: i64,
}

pub fn create_tables(database_connection: &Connection) {
    let query = format!("
    CREATE TABLE IF NOT EXISTS {TABLE_GENRES} ({COLUMN_NAME} TEXT PRIMARY KEY);

    CREATE TABLE IF NOT EXISTS {TABLE_ALBUM_ARTISTS} ({COLUMN_NAME} TEXT PRIMARY KEY, {COLUMN_GENRE} TEXT, {COLUMN_ALBUM_ARTIST_SORT_NAME} TEXT);

    CREATE TABLE IF NOT EXISTS {TABLE_ALBUMS} ({COLUMN_ID} TEXT PRIMARY KEY, {COLUMN_NAME} TEXT, {COLUMN_GENRE} TEXT, {COLUMN_ALBUM_ARTIST} TEXT, {COLUMN_ARTWORK_DATA} TEXT, {COLUMN_YEAR} INT);

    CREATE TABLE IF NOT EXISTS {TABLE_SONGS} ({COLUMN_ID} TEXT PRIMARY KEY, {COLUMN_NAME} TEXT, {COLUMN_GENRE} TEXT, {COLUMN_ALBUM_ARTIST} TEXT, {COLUMN_ALBUM} TEXT, {COLUMN_TRACK_NUMBER} INT, {COLUMN_ARTIST} TEXT, {COLUMN_FILE_PATH} TEXT, {COLUMN_DURATION} INT, {COLUMN_DISC_NUMBER} INT);
    ");
    database_connection.execute(query).unwrap();
}

pub fn process_song(database_connection: &Connection, song_file_path: PathBuf) {
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
                duration: &metadata.duration().unwrap_or_default(),
                disc_number: &metadata.disc_number().unwrap_or_default(),
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
        INSERT OR IGNORE INTO {TABLE_SONGS} VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}');
        "#,
        song.file_path,
        song.title,
        song.genre,
        song.album_artist,
        song.album,
        song.track_number,
        song.artist,
        song.file_path,
        song.duration,
        song.disc_number,
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
    let sort_name = get_sort_value_for_string(&song.album_artist);
    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_ALBUM_ARTISTS} VALUES ('{}', '{}','{}');
        "#,
        song.album_artist,
        song.genre,
        sort_name
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

fn get_sort_value_for_string(string: &str) -> String {
    let lowercase = &string.to_lowercase();
    if lowercase.starts_with("the ") {
        lowercase[4..].to_string()
    }
    else if lowercase.starts_with("a ") {
        lowercase[2..].to_string()
    }
    else if lowercase.starts_with("an ") {
        lowercase[3..].to_string()
    }
    else {
        lowercase.clone()
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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
                    ORDER BY {COLUMN_ALBUM_ARTIST_SORT_NAME}
                    "#,
                    escape_string_for_sql(&genre)
                ))
                .unwrap();

            album_artists.push(get_all_artists_string(&genre).to_string());

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

fn get_all_artists_string(genre: &str) -> &str {
    match genre {
        "Video Game" => return "All Games",
        _ => return "All Artists"
    }
}

#[allow(dead_code)]
pub fn get_albums_for_album_artist(album_artist: String, genre: String) -> Vec<String> {
    let mut albums = Vec::new();
    let database_connection = sqlite::open(DATABASE_PATH_MUSIC);
    match database_connection {
        Ok(database_connection) => {
            let mut statement: Statement<'_>;
            if !is_all_artists_string(&album_artist) {
                statement = database_connection
                    .prepare(format!(
                        r#"
                        SELECT * FROM {TABLE_ALBUMS}
                        WHERE {COLUMN_ALBUM_ARTIST} = '{}'
                        ORDER BY {COLUMN_YEAR}, {COLUMN_NAME}
                        "#,
                        escape_string_for_sql(&album_artist)
                    ))
                    .unwrap();
            }
            else {
                statement = database_connection
                    .prepare(format!(
                        r#"
                        SELECT {TABLE_ALBUMS}.{COLUMN_NAME}, {TABLE_ALBUMS}.{COLUMN_GENRE}
                        FROM {TABLE_ALBUMS}
                        INNER JOIN {TABLE_ALBUM_ARTISTS} ON {TABLE_ALBUMS}.{COLUMN_ALBUM_ARTIST} = {TABLE_ALBUM_ARTISTS}.{COLUMN_NAME}
                        WHERE {TABLE_ALBUMS}.{COLUMN_GENRE} = '{}'
                        ORDER BY {TABLE_ALBUM_ARTISTS}.{COLUMN_ALBUM_ARTIST_SORT_NAME}, {TABLE_ALBUMS}.{COLUMN_YEAR}, {TABLE_ALBUMS}.{COLUMN_NAME}
                        "#,
                        escape_string_for_sql(&genre)
                    )).unwrap()
            }

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

fn is_all_artists_string(string: &str) -> bool {
    match string {
        "All Artists" => return true,
        "All Games" => return true,
        _ => return false,
    }
}

#[allow(dead_code)]
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

            let mut artwork_source = "".to_string();
            let mut genre = "".to_string();
            let mut album_artist = "".to_string();
            let mut year = -1;
            while let Ok(State::Row) = statement.next() {
                artwork_source = statement.read::<String, _>(COLUMN_ARTWORK_DATA).unwrap();
                genre = statement.read::<String, _>(COLUMN_GENRE).unwrap();
                album_artist = statement.read::<String, _>(COLUMN_ALBUM_ARTIST).unwrap();
                year = statement.read::<i64, _>(COLUMN_YEAR).unwrap();
            }

            let tracks = get_tracks_for_album(&database_connection, album.clone());

            let mut duration_in_seconds = 0;
            for track in &tracks {
                duration_in_seconds += track.duration_in_seconds
            }
            
            Album {
                artwork_source,
                genre,
                album_artist,
                year,
                name: album.clone(),
                tracks,
                duration_in_seconds,
            }
        }
        Err(error) => {
            println!("Error connecting to database: {}", error);
            Album { 
                name: album, 
                artwork_source: "".to_string(), 
                genre: "".to_string(), 
                album_artist: "".to_string(), 
                year: -1, 
                tracks: Vec::new(),
                duration_in_seconds: 0,
            }
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
            ORDER BY {COLUMN_DISC_NUMBER},{COLUMN_TRACK_NUMBER}
            "#,
            escape_string_for_sql(&album)
        ))
        .unwrap();

    while let Ok(State::Row) = statement.next() {
        let name = statement.read::<String, _>(COLUMN_NAME).unwrap_or_default();
        let album_artist = statement.read::<String, _>(COLUMN_ALBUM_ARTIST).unwrap_or_default();
        let artist = statement.read::<String, _>(COLUMN_ARTIST).unwrap_or_default();
        let genre = statement.read::<String, _>(COLUMN_GENRE).unwrap_or_default();
        let artwork_source = statement.read::<String, _>(COLUMN_ARTWORK_DATA).unwrap_or_default();
        let file_path = statement.read::<String, _>(COLUMN_FILE_PATH).unwrap_or_default();
        let track_number = statement.read::<i64, _>(COLUMN_TRACK_NUMBER).unwrap_or_default();
        let duration_in_seconds = statement.read::<i64, _>(COLUMN_DURATION).unwrap_or_default();
        let disc_number = statement.read::<i64,_>(COLUMN_DISC_NUMBER).unwrap_or_default();

        tracks.push(Track { 
            name, 
            album_artist, 
            artist, 
            genre, 
            artwork_source, 
            file_path, 
            track_number,
            duration_in_seconds,
            disc_number,
        });
    }
    tracks
}