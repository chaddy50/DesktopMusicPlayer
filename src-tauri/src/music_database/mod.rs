use std::path::Path;

use album::Album;
use album_artist::AlbumArtist;
use base64::{engine::general_purpose, Engine as _};
use genre::Genre;
use sqlite::{Connection, State, Statement};
use track::Track;
use track_to_process::TrackToProcess;

pub mod album;
pub mod album_artist;
pub mod artist;
pub mod genre;
pub mod track;
pub mod track_to_process;

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

pub fn open_database_connection() -> Connection {
    sqlite::open(DATABASE_PATH_MUSIC).expect("Database connection should have been opened")
}

pub fn does_database_already_exist() -> bool {
    Path::new(DATABASE_PATH_MUSIC).exists()
}

pub fn create_tables(database_connection: &Connection) {
    let query = format!(
        r#"
    CREATE TABLE IF NOT EXISTS {TABLE_GENRES} ({COLUMN_ID} INTEGER PRIMARY KEY AUTOINCREMENT, {COLUMN_NAME} TEXT);

    CREATE TABLE IF NOT EXISTS {TABLE_ALBUM_ARTISTS} ({COLUMN_ID} INTEGER PRIMARY KEY AUTOINCREMENT, {COLUMN_NAME} TEXT, {COLUMN_GENRE_ID} INTEGER, {COLUMN_ALBUM_ARTIST_SORT_NAME} TEXT);

    CREATE TABLE IF NOT EXISTS {TABLE_ARTISTS} ({COLUMN_ID} INTEGER PRIMARY KEY AUTOINCREMENT, {COLUMN_NAME} TEXT);

    CREATE TABLE IF NOT EXISTS {TABLE_ALBUMS} ({COLUMN_ID} INTEGER PRIMARY KEY AUTOINCREMENT, {COLUMN_NAME} TEXT, {COLUMN_GENRE_ID} INTEGER, {COLUMN_ALBUM_ARTIST_ID} INTEGER, {COLUMN_ARTWORK_DATA} TEXT, {COLUMN_YEAR} INT);

    CREATE TABLE IF NOT EXISTS {TABLE_SONGS} ({COLUMN_ID} INTEGER PRIMARY KEY AUTOINCREMENT, {COLUMN_NAME} TEXT, {COLUMN_GENRE_ID} INTEGER, {COLUMN_ALBUM_ARTIST_ID} INTEGER, {COLUMN_ALBUM_ID} INTEGER, {COLUMN_TRACK_NUMBER} INT, {COLUMN_ARTIST_ID} TEXT, {COLUMN_FILE_PATH} TEXT, {COLUMN_DURATION} INT, {COLUMN_DISC_NUMBER} INT);
    "#
    );
    database_connection.execute(query).unwrap();
}

pub fn add_track_to_database(
    database_connection: &Connection,
    track_to_process: &TrackToProcess,
    genre_id: &i64,
    album_artist_id: &i64,
    album_id: &i64,
    artist_id: &i64,
) {
    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_SONGS} ({COLUMN_NAME}, {COLUMN_GENRE_ID}, {COLUMN_ALBUM_ARTIST_ID}, {COLUMN_ALBUM_ID}, {COLUMN_TRACK_NUMBER}, {COLUMN_ARTIST_ID}, {COLUMN_FILE_PATH}, {COLUMN_DURATION}, {COLUMN_DISC_NUMBER}) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}');
        "#,
        track_to_process.title,
        genre_id,
        album_artist_id,
        album_id,
        track_to_process.track_number,
        artist_id,
        track_to_process.file_path,
        track_to_process.duration,
        track_to_process.disc_number,
    );
    try_execute_insert_query(
        database_connection,
        &query,
        "track",
        format!("{}: {}", track_to_process.title, track_to_process.file_path).as_str(),
    );
}

pub fn add_album_to_database(
    database_connection: &Connection,
    track_to_process: &TrackToProcess,
    genre_id: &i64,
    album_artist_id: &i64,
) -> i64 {
    let mime_type = track_to_process.artwork.mime_type;
    let cover_data = track_to_process.artwork.data;

    let mut artwork_data = "NO_ARTWORK".to_string();
    if cover_data != [1] {
        let cover_data = convert_artwork_data_to_base_64(cover_data);
        artwork_data = format!("data:image/{:?};base64,{}", mime_type, &cover_data);
    }

    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_ALBUMS} ({COLUMN_NAME},{COLUMN_GENRE_ID},{COLUMN_ALBUM_ARTIST_ID},{COLUMN_ARTWORK_DATA},{COLUMN_YEAR}) VALUES ('{}', '{}', '{}', '{}', '{}');
        "#,
        track_to_process.album, genre_id, album_artist_id, artwork_data, track_to_process.year,
    );
    try_execute_insert_query(
        database_connection,
        &query,
        "album",
        &track_to_process.album,
    )
}

pub fn add_album_artist_to_database(
    database_connection: &Connection,
    track_to_process: &TrackToProcess,
    genre_id: &i64,
) -> i64 {
    let sort_name = get_sort_value_for_string(&track_to_process.album_artist);
    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_ALBUM_ARTISTS} ({COLUMN_NAME}, {COLUMN_GENRE_ID}, {COLUMN_ALBUM_ARTIST_SORT_NAME}) VALUES ('{}', '{}','{}');
        "#,
        track_to_process.album_artist, genre_id, sort_name
    );
    try_execute_insert_query(
        database_connection,
        &query,
        "album artist",
        &track_to_process.album_artist,
    )
}

pub fn add_genre_to_database(
    database_connection: &Connection,
    track_to_process: &TrackToProcess,
) -> i64 {
    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_GENRES} ({COLUMN_NAME}) VALUES ('{}');
        "#,
        track_to_process.genre
    );
    try_execute_insert_query(
        database_connection,
        &query,
        "genre",
        &track_to_process.genre,
    )
}

pub fn add_artist_to_database(
    database_connection: &Connection,
    track_to_process: &TrackToProcess,
) -> i64 {
    let query = format!(
        r#"
        INSERT OR IGNORE INTO {TABLE_ARTISTS} ({COLUMN_NAME}) VALUES ('{}');
        "#,
        track_to_process.artist
    );
    try_execute_insert_query(
        database_connection,
        &query,
        "artist",
        &track_to_process.artist,
    )
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
        let name = statement.read::<String, _>(COLUMN_NAME).unwrap_or_default();
        genres.push(Genre::new(id, name));
    }

    genres
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
    album_artists.push(AlbumArtist::new(
        0,
        get_all_artists_name(&genre_name).to_string(),
    ));

    while let Ok(State::Row) = statement.next() {
        let id = statement.read::<i64, _>(COLUMN_ID).unwrap_or(-1);
        let name = statement.read::<String, _>(COLUMN_NAME).unwrap_or_default();
        album_artists.push(AlbumArtist::new(id, name));
    }
    album_artists
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
    } else {
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
        let artwork_source = statement
            .read::<String, _>(COLUMN_ARTWORK_DATA)
            .unwrap_or_default();
        let genre_name = statement
            .read::<String, _>("genre_name")
            .unwrap_or_default();
        let album_artist_name = statement
            .read::<String, _>("album_artist_name")
            .unwrap_or_default();

        let tracks = get_tracks_for_album(&database_connection, &id);

        let mut duration_in_seconds = 0;
        for track in &tracks {
            duration_in_seconds += track.duration_in_seconds
        }

        albums.push(Album::new(
            id,
            name,
            *album_artist_id,
            album_artist_name,
            *genre_id,
            genre_name,
            artwork_source,
            year,
            tracks,
            duration_in_seconds,
        ));
    }

    albums
}

fn get_tracks_for_album(database_connection: &Connection, album_id: &i64) -> Vec<Track> {
    let mut tracks = Vec::new();
    let mut statement = database_connection
        .prepare(format!(
            r#"
            SELECT {TABLE_SONGS}.{COLUMN_NAME}, {TABLE_SONGS}.{COLUMN_ALBUM_ARTIST_ID}, {TABLE_SONGS}.{COLUMN_ARTIST_ID}, {TABLE_SONGS}.{COLUMN_GENRE_ID}, {TABLE_SONGS}.{COLUMN_FILE_PATH}, {TABLE_SONGS}.{COLUMN_TRACK_NUMBER}, {TABLE_SONGS}.{COLUMN_DURATION}, {TABLE_SONGS}.{COLUMN_DISC_NUMBER}, {TABLE_ALBUM_ARTISTS}.{COLUMN_NAME} AS album_artist_name, {TABLE_ARTISTS}.{COLUMN_NAME} AS artist_name, {TABLE_GENRES}.{COLUMN_NAME} AS genre_name, {TABLE_ALBUMS}.{COLUMN_NAME} as album_name
            FROM {TABLE_SONGS}
            INNER JOIN {TABLE_ALBUM_ARTISTS} ON {TABLE_SONGS}.{COLUMN_ALBUM_ARTIST_ID} = {TABLE_ALBUM_ARTISTS}.{COLUMN_ID}
            INNER JOIN {TABLE_ARTISTS} ON {TABLE_SONGS}.{COLUMN_ARTIST_ID} = {TABLE_ARTISTS}.{COLUMN_ID}
            INNER JOIN {TABLE_GENRES} ON {TABLE_SONGS}.{COLUMN_GENRE_ID} = {TABLE_GENRES}.{COLUMN_ID}
            INNER JOIN {TABLE_ALBUMS} ON {TABLE_SONGS}.{COLUMN_ALBUM_ID} = {TABLE_ALBUMS}.{COLUMN_ID}
            WHERE {COLUMN_ALBUM_ID} = '{}'
            ORDER BY {COLUMN_DISC_NUMBER},{COLUMN_TRACK_NUMBER}
            "#,
            album_id
        ))
        .unwrap();

    while let Ok(State::Row) = statement.next() {
        let name = statement.read::<String, _>(COLUMN_NAME).unwrap_or_default();
        let album_artist_id = statement
            .read::<i64, _>(COLUMN_ALBUM_ARTIST_ID)
            .unwrap_or_default();
        let album_artist_name = statement
            .read::<String, _>("album_artist_name")
            .unwrap_or_default();
        let artist_id = statement
            .read::<i64, _>(COLUMN_ARTIST_ID)
            .unwrap_or_default();
        let artist_name = statement
            .read::<String, _>("artist_name")
            .unwrap_or_default();
        let genre_id = statement
            .read::<i64, _>(COLUMN_GENRE_ID)
            .unwrap_or_default();
        let genre_name = statement
            .read::<String, _>("genre_name")
            .unwrap_or_default();
        let file_path = statement
            .read::<String, _>(COLUMN_FILE_PATH)
            .unwrap_or_default();
        let track_number = statement.read::<i64, _>(COLUMN_TRACK_NUMBER).unwrap_or(-1);
        let duration_in_seconds = statement.read::<i64, _>(COLUMN_DURATION).unwrap_or(-1);
        let disc_number = statement.read::<i64, _>(COLUMN_DISC_NUMBER).unwrap_or(-1);
        let album_name = statement
            .read::<String, _>("album_name")
            .unwrap_or_default();

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
            album_name,
        ));
    }
    tracks
}

fn try_execute_insert_query(
    database_connection: &Connection,
    query: &str,
    row_descriptor: &str,
    row_details: &str,
) -> i64 {
    let result = database_connection.execute(query);
    match result {
        Ok(_result) => get_id_of_last_inserted_row(database_connection),
        Err(error) => {
            println!("Error adding {row_descriptor} to database: {row_details}");
            println!("     ERROR: {}", error);
            -1
        }
    }
}

fn get_id_of_last_inserted_row(database_connection: &Connection) -> i64 {
    let mut statement = database_connection
        .prepare(
            r#"
            SELECT last_insert_rowid()
            "#,
        )
        .unwrap();

    let mut last_id = -1;
    while let Ok(State::Row) = statement.next() {
        last_id = statement.read::<i64, _>("last_insert_rowid()").unwrap();
    }

    last_id
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
        name = statement.read::<String, _>(COLUMN_NAME).unwrap_or_default();
    }

    name
}

fn get_sort_value_for_string(string: &str) -> String {
    let lowercase = string.to_lowercase();
    if lowercase.starts_with("the ") {
        lowercase.strip_prefix("the ").unwrap().to_string()
    } else if lowercase.starts_with("a ") {
        lowercase.strip_prefix("a ").unwrap().to_string()
    } else if lowercase.starts_with("an ") {
        lowercase.strip_prefix("an ").unwrap().to_string()
    } else {
        lowercase
    }
}

fn convert_artwork_data_to_base_64(artwork_data: &[u8]) -> String {
    general_purpose::STANDARD.encode(artwork_data)
}

pub fn escape_string_for_sql(str: &str) -> String {
    str.replace('\'', "\'\'")
}

fn get_all_artists_name(genre_name: &str) -> &str {
    match genre_name.to_lowercase().as_str() {
        "video game" => "All Games",
        _ => "All Artists",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_sort_value_the() {
        assert_eq!(get_sort_value_for_string("the beatles"), "beatles");
        assert_eq!(get_sort_value_for_string("THE BEATLES"), "beatles");
    }

    #[test]
    fn get_sort_value_a() {
        assert_eq!(
            get_sort_value_for_string("a day to remember"),
            "day to remember"
        );
        assert_eq!(
            get_sort_value_for_string("A DAY TO REMEMBER"),
            "day to remember"
        );
    }

    #[test]
    fn get_sort_value_an() {
        assert_eq!(get_sort_value_for_string("an orchestra"), "orchestra");
        assert_eq!(get_sort_value_for_string("AN ORCHESTRA"), "orchestra");
    }

    #[test]
    fn get_all_artists_name_rock() {
        assert_eq!(get_all_artists_name("rock"), "All Artists");
        assert_eq!(get_all_artists_name("Rock"), "All Artists");
    }

    #[test]
    fn get_all_artists_name_video_game() {
        assert_eq!(get_all_artists_name("video game"), "All Games");
        assert_eq!(get_all_artists_name("Video Game"), "All Games");
    }
}
