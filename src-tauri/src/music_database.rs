use std::path::Path;

use audiotags::Picture;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use sqlite::{Connection, State, Statement};

const DATABASE_PATH_MUSIC: &str = "music_database.db";

const TABLE_SONGS: &str = "songs";
const TABLE_ALBUMS: &str = "albums";
const TABLE_ALBUM_ARTISTS: &str = "album_artists";
const TABLE_ARTISTS: &str = "artists";
const TABLE_GENRES: &str = "genres";
const COLUMN_ID: &str = "id";
const COLUMN_NAME: &str = "name";
const COLUMN_GENRE_ID: &str = "genre_id";
const COLUMN_ALBUM_ARTIST_ID: &str = "album_artist_id";
const COLUMN_ARTIST_ID: &str = "artist_id";
const COLUMN_ARTWORK_DATA: &str = "artwork_data";
const COLUMN_ALBUM_ID: &str = "album_id";
const COLUMN_YEAR: &str = "year";
const COLUMN_TRACK_NUMBER: &str = "track_number";
const COLUMN_FILE_PATH: &str = "file_path";
const COLUMN_DURATION: &str = "duration";
const COLUMN_ALBUM_ARTIST_SORT_NAME: &str = "album_artist_sort_name";
const COLUMN_DISC_NUMBER: &str = "disc_number";

#[derive(Clone)]
pub struct TrackToProcess<'a> {
    title: String,
    album: String,
    album_artist: String,
    artist: String,
    genre: String,
    artwork: Picture<'a>,
    file_path: String,
    year: i32,
    track_number: u16,
    duration: f64,
    disc_number: u16,
}

impl<'a> TrackToProcess<'a> {
    pub fn new(title: &str, album: &str, album_artist: &str, artist: &str, genre: &str, artwork: &Picture<'a>, file_path: &str, year: &i32, track_number: &u16, duration: &f64, disc_number: &u16) -> TrackToProcess<'a> {
        TrackToProcess {
            title: escape_string_for_sql(title),
            album: escape_string_for_sql(album),
            album_artist: escape_string_for_sql(album_artist),
            artist: escape_string_for_sql(artist),
            genre: escape_string_for_sql(genre),
            artwork: artwork.clone(),
            file_path: escape_string_for_sql(file_path),
            year: year.clone(),
            track_number: track_number.clone(),
            duration: duration.clone(),
            disc_number: disc_number.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NowPlayingQueue {
    pub now_playing_tracks: Vec<Track>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Track {
    pub name: String,
    album_artist_id: i64,
    album_artist_name: String,
    artist_id: i64,
    artist_name: String,
    genre_id: i64,
    genre_name: String,
    pub file_path: String,
    track_number: i64,
    disc_number: i64,
    duration_in_seconds: i64,
}

impl Track {
    pub fn new(name: String, album_artist_id: i64, album_artist_name: String, artist_id: i64, artist_name: String, genre_id: i64, genre_name: String, file_path: String, track_number: i64, disc_number: i64, duration_in_seconds: i64) -> Track {
        Track {
            name,
            album_artist_id,
            album_artist_name,
            artist_id,
            artist_name,
            genre_id,
            genre_name,
            file_path,
            track_number,
            disc_number,
            duration_in_seconds
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Album {
    id: i64,
    pub name: String,
    album_artist_id: i64,
    album_artist_name: String,
    genre_id: i64,
    genre_name: String,
    artwork_source: String,
    year: i64,
    pub tracks: Vec<Track>,
    duration_in_seconds: i64,
}

impl Album {
    pub fn new(id: i64, name: String, album_artist_id: i64, album_artist_name: String, genre_id: i64, genre_name: String, artwork_source: String, year: i64, tracks: Vec<Track>, duration_in_seconds: i64) -> Self {
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
            duration_in_seconds
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AlbumArtist {
    id: i64,
    pub name: String,
}

impl AlbumArtist {
    pub fn new(id: i64, name: String) -> AlbumArtist {
        AlbumArtist {
            id,
            name
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Genre {
    id: i64,
    pub name: String,
}

impl Genre {
    pub fn new(id: i64, name: String) -> Self {
        Genre {
            id,
            name
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Artist {
    pub name: String,
}

pub fn open_database_connection() -> Connection {
    sqlite::open(DATABASE_PATH_MUSIC).expect("Database connection should have been opened")
}

pub fn does_database_already_exist() -> bool {
    Path::new(DATABASE_PATH_MUSIC).exists()
}

pub fn create_tables(database_connection: &Connection) {
    let query = format!(r#"
    CREATE TABLE IF NOT EXISTS {TABLE_GENRES} ({COLUMN_ID} INTEGER PRIMARY KEY AUTOINCREMENT, {COLUMN_NAME} TEXT);

    CREATE TABLE IF NOT EXISTS {TABLE_ALBUM_ARTISTS} ({COLUMN_ID} INTEGER PRIMARY KEY AUTOINCREMENT, {COLUMN_NAME} TEXT, {COLUMN_GENRE_ID} INTEGER, {COLUMN_ALBUM_ARTIST_SORT_NAME} TEXT);

    CREATE TABLE IF NOT EXISTS {TABLE_ARTISTS} ({COLUMN_ID} INTEGER PRIMARY KEY AUTOINCREMENT, {COLUMN_NAME} TEXT);

    CREATE TABLE IF NOT EXISTS {TABLE_ALBUMS} ({COLUMN_ID} INTEGER PRIMARY KEY AUTOINCREMENT, {COLUMN_NAME} TEXT, {COLUMN_GENRE_ID} INTEGER, {COLUMN_ALBUM_ARTIST_ID} INTEGER, {COLUMN_ARTWORK_DATA} TEXT, {COLUMN_YEAR} INT);

    CREATE TABLE IF NOT EXISTS {TABLE_SONGS} ({COLUMN_ID} INTEGER PRIMARY KEY AUTOINCREMENT, {COLUMN_NAME} TEXT, {COLUMN_GENRE_ID} INTEGER, {COLUMN_ALBUM_ARTIST_ID} INTEGER, {COLUMN_ALBUM_ID} INTEGER, {COLUMN_TRACK_NUMBER} INT, {COLUMN_ARTIST_ID} TEXT, {COLUMN_FILE_PATH} TEXT, {COLUMN_DURATION} INT, {COLUMN_DISC_NUMBER} INT);
    "#);
    database_connection.execute(query).unwrap();
}

pub fn add_track_to_database(database_connection: &Connection, song: &TrackToProcess, genre_id: &i64, album_artist_id: &i64, album_id: &i64, artist_id: &i64) {
    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_SONGS} ({COLUMN_NAME}, {COLUMN_GENRE_ID}, {COLUMN_ALBUM_ARTIST_ID}, {COLUMN_ALBUM_ID}, {COLUMN_TRACK_NUMBER}, {COLUMN_ARTIST_ID}, {COLUMN_FILE_PATH}, {COLUMN_DURATION}, {COLUMN_DISC_NUMBER}) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}');
        "#,
        song.title,
        genre_id,
        album_artist_id,
        album_id,
        song.track_number,
        artist_id,
        song.file_path,
        song.duration,
        song.disc_number,
    );
    let result = database_connection.execute(query);
    match result {
        Ok(_result) => {}
        Err(error) => {
            println!("Error adding song to database: {} - {}", song.title, song.file_path);
            println!("     ERROR: {}", error);
        }
    }
}

pub fn add_album_to_database(database_connection: &Connection, song: &TrackToProcess, genre_id: &i64, album_artist_id: &i64) {
    let mime_type = song.artwork.mime_type;
    let cover_data = song.artwork.data;

    let mut artwork_data = "NO_ARTWORK".to_string();
    if cover_data != &[1] {
        let cover_data = convert_artwork_data_to_base_64(cover_data);
        artwork_data = format!("data:image/{:?};base64,{}", mime_type, &cover_data);
    }

    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_ALBUMS} ({COLUMN_NAME},{COLUMN_GENRE_ID},{COLUMN_ALBUM_ARTIST_ID},{COLUMN_ARTWORK_DATA},{COLUMN_YEAR}) VALUES ('{}', '{}', '{}', '{}', '{}');
        "#,
        song.album,
        genre_id,
        album_artist_id,
        artwork_data,
        song.year,
    );

    let result = database_connection.execute(query);
    match result {
        Ok(_result) => {}
        Err(error) => {
            println!("Error adding album to database: {}", song.album);
            println!("     ERROR: {}", error);
        }
    }
}

pub fn add_album_artist_to_database(database_connection: &Connection, song: &TrackToProcess, genre_id: &i64) {
    let sort_name = get_sort_value_for_string(&song.album_artist);
    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_ALBUM_ARTISTS} ({COLUMN_NAME}, {COLUMN_GENRE_ID}, {COLUMN_ALBUM_ARTIST_SORT_NAME}) VALUES ('{}', '{}','{}');
        "#,
        song.album_artist,
        genre_id,
        sort_name
    );
    let result = database_connection.execute(query);
    match result {
        Ok(_result) => {}
        Err(error) => {
            println!("Error adding album artist to database: {}", song.album_artist);
            println!("     ERROR: {}", error);
        }
    }
}

pub fn add_genre_to_database(database_connection: &Connection, song: &TrackToProcess) {
    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_GENRES} ({COLUMN_NAME}) VALUES ('{}');
        "#,
        song.genre
    );
    let result = database_connection.execute(query);
    match result {
        Ok(_result) => {}
        Err(error) => {
            println!("Error adding genre to database: {}", song.genre);
            println!("     ERROR: {}", error);
        }
    }
}

pub fn add_artist_to_database(database_connection: &Connection, song: &TrackToProcess) {
    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_ARTISTS} ({COLUMN_NAME}) VALUES ('{}');
        "#,
        song.artist
    );
    let result = database_connection.execute(query);
    match result {
        Ok(_result) => {}
        Err(error) => {
            println!("Error adding artist to database: {}", song.genre);
            println!("     ERROR: {}", error);
        }
    }
}

pub fn get_genres() -> Vec<Genre> {
    let database_connection = open_database_connection();

    let mut statement = database_connection
        .prepare(format!(
            r#"
        SELECT {COLUMN_ID}, {COLUMN_NAME} FROM {TABLE_GENRES} 
        ORDER BY {COLUMN_NAME}
        "#
        ))
        .unwrap();

    let mut genres = Vec::new();
    while let Ok(State::Row) = statement.next() {
        let id = statement.read::<i64, _>(COLUMN_ID).unwrap_or(-1);
        let name = statement.read::<String,_>(COLUMN_NAME).unwrap_or_default();
        genres.push(Genre::new(id, name));
    }

    genres
}

pub fn get_genre_id(genre_name: &str) -> i64 {
    let database_connection = open_database_connection();

    let mut statement = database_connection
        .prepare(format!(
            r#"
            SELECT {COLUMN_ID} FROM {TABLE_GENRES} 
            WHERE {COLUMN_NAME} = '{}'
            "#,
            escape_string_for_sql(genre_name))
        )
        .unwrap();

    let mut genre_id: i64 = -1;
    while let Ok(State::Row) = statement.next() {
        genre_id = statement.read::<i64, _>(COLUMN_ID).unwrap();
    }

    genre_id
}

pub fn get_album_artist_id(album_artist_name: &str) -> i64 {
    let database_connection = open_database_connection();

    let mut statement = database_connection
        .prepare(format!(
            r#"
            SELECT {COLUMN_ID} FROM {TABLE_ALBUM_ARTISTS} 
            WHERE {COLUMN_NAME} = '{}'
            "#,
            escape_string_for_sql(album_artist_name))
        )
        .unwrap();

    let mut album_artist_id: i64 = -1;
    while let Ok(State::Row) = statement.next() {
        album_artist_id = statement.read::<i64, _>(COLUMN_ID).unwrap();
    }

    album_artist_id
}

pub fn get_album_id(album_name: &str, album_artist_id: &i64) -> i64 {
    let database_connection = open_database_connection();

    let mut statement = database_connection
        .prepare(format!(
            r#"
            SELECT {COLUMN_ID} FROM {TABLE_ALBUMS} 
            WHERE {COLUMN_NAME} = '{}' AND {COLUMN_ALBUM_ARTIST_ID} = '{}'
            "#,
            escape_string_for_sql(album_name),
            album_artist_id)
        )
        .unwrap();

    let mut album_id: i64 = -1;
    while let Ok(State::Row) = statement.next() {
        album_id = statement.read::<i64, _>(COLUMN_ID).unwrap();
    }

    album_id
}

pub fn get_artist_id(artist_name: &str) -> i64 {
    let database_connection = open_database_connection();

    let mut statement = database_connection
        .prepare(format!(
            r#"
            SELECT {COLUMN_ID} FROM {TABLE_ARTISTS} 
            WHERE {COLUMN_NAME} = '{}'
            "#,
            escape_string_for_sql(artist_name))
        )
        .unwrap();

    let mut artist_id: i64 = -1;
    while let Ok(State::Row) = statement.next() {
        artist_id = statement.read::<i64, _>(COLUMN_ID).unwrap();
    }

    artist_id
}

pub fn get_album_artists_for_genre(genre_id: &i64) -> Vec<AlbumArtist> {
    let database_connection = open_database_connection();

    let mut statement = database_connection
        .prepare(format!(
            r#"
            SELECT {COLUMN_ID}, {COLUMN_NAME} 
            FROM {TABLE_ALBUM_ARTISTS} 
            WHERE {COLUMN_GENRE_ID} = '{}' AND {COLUMN_NAME} <> ""
            ORDER BY {COLUMN_ALBUM_ARTIST_SORT_NAME}
            "#,
            genre_id
        ))
        .unwrap();

    let mut album_artists = Vec::new();
    let genre_name = get_genre_name(genre_id);
    album_artists.push(AlbumArtist::new(0, get_all_artists_name(&genre_name).to_string()));

    while let Ok(State::Row) = statement.next() {
        let id = statement.read::<i64, _>(COLUMN_ID).unwrap_or(-1);
        let name = statement.read::<String,_>(COLUMN_NAME).unwrap_or_default();
        album_artists.push(AlbumArtist::new(id, name));
    }
    album_artists
}

fn get_genre_name(genre_id: &i64) -> String {
    let database_connection = open_database_connection();

    let mut statement = database_connection
        .prepare(format!(
            r#"
            SELECT {COLUMN_NAME} 
            FROM {TABLE_GENRES} 
            WHERE {COLUMN_ID} = '{}'
        "#,
        genre_id
        ))
        .unwrap();

    let mut name = "".to_string();
    while let Ok(State::Row) = statement.next() {
        name = statement.read::<String,_>(COLUMN_NAME).unwrap_or_default();
    }

    name
}

pub fn get_albums_for_album_artist(album_artist_id: &i64, genre_id: &i64) -> Vec<Album> {
    let database_connection = open_database_connection();

    let mut statement: Statement<'_>;
    if *album_artist_id != 0 {
        statement = database_connection
            .prepare(format!(
                r#"
                SELECT {TABLE_ALBUMS}.{COLUMN_ID}, {TABLE_ALBUMS}.{COLUMN_NAME}, {TABLE_ALBUMS}.{COLUMN_YEAR}, {TABLE_ALBUMS}.{COLUMN_ARTWORK_DATA}, {TABLE_GENRES}.{COLUMN_NAME} AS genre_name, {TABLE_ALBUM_ARTISTS}.{COLUMN_NAME} AS album_artist_name
                FROM {TABLE_ALBUMS}
                INNER JOIN {TABLE_GENRES} ON {TABLE_ALBUMS}.{COLUMN_GENRE_ID} = {TABLE_GENRES}.{COLUMN_ID}
                INNER JOIN {TABLE_ALBUM_ARTISTS} ON {TABLE_ALBUMS}.{COLUMN_ALBUM_ARTIST_ID} = {TABLE_ALBUM_ARTISTS}.{COLUMN_ID}
                WHERE {COLUMN_ALBUM_ARTIST_ID} = '{}'
                ORDER BY {TABLE_ALBUMS}.{COLUMN_YEAR}, {TABLE_ALBUMS}.{COLUMN_NAME}
                "#,
                album_artist_id
            ))
            .unwrap();
    }
    else {
        statement = database_connection
            .prepare(format!(
                r#"
                SELECT {TABLE_ALBUMS}.{COLUMN_ID}, {TABLE_ALBUMS}.{COLUMN_NAME}, {TABLE_ALBUMS}.{COLUMN_YEAR}, {TABLE_ALBUMS}.{COLUMN_ARTWORK_DATA}, {TABLE_GENRES}.{COLUMN_NAME} AS genre_name, {TABLE_ALBUM_ARTISTS}.{COLUMN_NAME} AS album_artist_name
                FROM {TABLE_ALBUMS}
                INNER JOIN {TABLE_GENRES} ON {TABLE_ALBUMS}.{COLUMN_GENRE_ID} = {TABLE_GENRES}.{COLUMN_ID}
                INNER JOIN {TABLE_ALBUM_ARTISTS} ON {TABLE_ALBUMS}.{COLUMN_ALBUM_ARTIST_ID} = {TABLE_ALBUM_ARTISTS}.{COLUMN_ID}
                WHERE {TABLE_ALBUMS}.{COLUMN_GENRE_ID} = '{}'
                ORDER BY {TABLE_ALBUM_ARTISTS}.{COLUMN_ALBUM_ARTIST_SORT_NAME}, {TABLE_ALBUMS}.{COLUMN_YEAR}, {TABLE_ALBUMS}.{COLUMN_NAME}
                "#,
                genre_id
            )).unwrap();
    }

    let mut albums = Vec::new();
    while let Ok(State::Row) = statement.next() {
        let id = statement.read::<i64, _>(COLUMN_ID).unwrap_or(-1);
        let name = statement.read::<String, _>(COLUMN_NAME).unwrap_or_default();
        let year = statement.read::<i64, _>(COLUMN_YEAR).unwrap_or(-1);
        let artwork_source = statement.read::<String, _>(COLUMN_ARTWORK_DATA).unwrap_or_default();
        let genre_name = statement.read::<String, _>("genre_name").unwrap_or_default();
        let album_artist_name = statement.read::<String, _>("album_artist_name").unwrap_or_default();

        let tracks = get_tracks_for_album(&database_connection, &id);
    
        let mut duration_in_seconds = 0;
        for track in &tracks {
            duration_in_seconds += track.duration_in_seconds
        }

        println!("tracks for album {}({}): {:?}", name, id, tracks.len());

        albums.push(Album::new(id, name, *album_artist_id, album_artist_name, *genre_id, genre_name, artwork_source, year, tracks, duration_in_seconds));
    }

    albums
}

fn get_tracks_for_album(database_connection: &Connection, album_id: &i64) -> Vec<Track> {
    let mut tracks = Vec::new();
    let mut statement = database_connection
        .prepare(format!(
            r#"
            SELECT {TABLE_SONGS}.{COLUMN_NAME}, {TABLE_SONGS}.{COLUMN_ALBUM_ARTIST_ID}, {TABLE_SONGS}.{COLUMN_ARTIST_ID}, {TABLE_SONGS}.{COLUMN_GENRE_ID}, {TABLE_SONGS}.{COLUMN_FILE_PATH}, {TABLE_SONGS}.{COLUMN_TRACK_NUMBER}, {TABLE_SONGS}.{COLUMN_DURATION}, {TABLE_SONGS}.{COLUMN_DISC_NUMBER}, {TABLE_ALBUM_ARTISTS}.{COLUMN_NAME} AS album_artist_name, {TABLE_ARTISTS}.{COLUMN_NAME} AS artist_name, {TABLE_GENRES}.{COLUMN_NAME} AS genre_name
            FROM {TABLE_SONGS}
            INNER JOIN {TABLE_ALBUM_ARTISTS} ON {TABLE_SONGS}.{COLUMN_ALBUM_ARTIST_ID} = {TABLE_ALBUM_ARTISTS}.{COLUMN_ID}
            INNER JOIN {TABLE_ARTISTS} ON {TABLE_SONGS}.{COLUMN_ARTIST_ID} = {TABLE_ARTISTS}.{COLUMN_ID}
            INNER JOIN {TABLE_GENRES} ON {TABLE_SONGS}.{COLUMN_GENRE_ID} = {TABLE_GENRES}.{COLUMN_ID}
            WHERE {COLUMN_ALBUM_ID} = '{}'
            ORDER BY {COLUMN_DISC_NUMBER},{COLUMN_TRACK_NUMBER}
            "#,
            album_id
        ))
        .unwrap();

    while let Ok(State::Row) = statement.next() {
        let name = statement.read::<String, _>(COLUMN_NAME).unwrap_or_default();
        let album_artist_id = statement.read::<i64, _>(COLUMN_ALBUM_ARTIST_ID).unwrap_or_default();
        let album_artist_name = statement.read::<String, _>("album_artist_name").unwrap_or_default();
        let artist_id = statement.read::<i64, _>(COLUMN_ARTIST_ID).unwrap_or_default();
        let artist_name = statement.read::<String,_>("artist_name").unwrap_or_default();
        let genre_id = statement.read::<i64, _>(COLUMN_GENRE_ID).unwrap_or_default();
        let genre_name = statement.read::<String,_>("genre_name").unwrap_or_default();
        let file_path = statement.read::<String, _>(COLUMN_FILE_PATH).unwrap_or_default();
        let track_number = statement.read::<i64, _>(COLUMN_TRACK_NUMBER).unwrap_or(-1);
        let duration_in_seconds = statement.read::<i64, _>(COLUMN_DURATION).unwrap_or(-1);
        let disc_number = statement.read::<i64,_>(COLUMN_DISC_NUMBER).unwrap_or(-1);

        tracks.push(Track::new(
            name, 
            album_artist_id, 
            album_artist_name,
            artist_id, 
            artist_name,
            genre_id,
            genre_name,
            file_path, 
            track_number,
            disc_number,
            duration_in_seconds,
        ));
    }
    tracks
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

fn convert_artwork_data_to_base_64(artwork_data: &[u8]) -> String {
    general_purpose::STANDARD.encode(artwork_data)
}

fn escape_string_for_sql(str: &str) -> String {
    str.replace('\'', "\'\'")
}

fn get_all_artists_name(genre_name: &str) -> &str {
    match genre_name {
        "Video Game" => return "All Games",
        _ => return "All Artists"
    };
}