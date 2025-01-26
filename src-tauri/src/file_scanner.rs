use std::{fs::{self, DirEntry}, path::PathBuf};

use audiotags::{Picture, Tag};
use music_player_lib::music_database::{self, TrackToProcess};
use sqlite::Connection;

pub fn build_music_database() {
    if music_database::does_database_already_exist() {
        return;
    }

    let database_connection = music_database::open_database_connection();

    music_database::create_tables(&database_connection);

    let mut processed_artists: Vec<String> = Vec::new();
    let mut processed_albums: Vec<String> = Vec::new();
    let mut processed_album_artists: Vec<String> = Vec::new();
    let mut processed_genres: Vec<String> = Vec::new();

    scan_directory(&database_connection, "/home/nathan/Music/Video Game", &mut processed_albums, &mut processed_album_artists, &mut processed_genres, &mut processed_artists);
    scan_directory(&database_connection, "/home/nathan/Music/Rock", &mut processed_albums, &mut processed_album_artists, &mut processed_genres, &mut processed_artists);
    scan_directory(&database_connection, "/home/nathan/Music/Jazz", &mut processed_albums, &mut processed_album_artists, &mut processed_genres, &mut processed_artists);
    scan_directory(&database_connection, "/home/nathan/Music/Classic Rock", &mut processed_albums, &mut processed_album_artists, &mut processed_genres, &mut processed_artists);
    scan_directory(&database_connection, "/home/nathan/Music/Ambient", &mut processed_albums, &mut processed_album_artists, &mut processed_genres, &mut processed_artists);
    scan_directory(&database_connection, "/home/nathan/Music/Electronic", &mut processed_albums, &mut processed_album_artists, &mut processed_genres, &mut processed_artists);
}

fn scan_directory(database_connection: &Connection, directory_path: &str, processed_albums: &mut Vec<String>, processed_album_artists: &mut Vec<String>, processed_genres: &mut Vec<String>, processed_artists: &mut Vec<String>) {

    let directory_entries = fs::read_dir(directory_path).expect(format!("Directory entries should have been read: {}", directory_path).as_str());
    
    for directory_entry_result in directory_entries {
        let directory_entry = directory_entry_result.expect("Directory entry should have been read");
        scan_directory_entry(database_connection, &directory_entry, processed_albums, processed_album_artists, processed_genres, processed_artists);
    }
}

fn scan_directory_entry(database_connection: &Connection, directory_entry: &DirEntry, processed_albums: &mut Vec<String>, processed_album_artists: &mut Vec<String>, processed_genres: &mut Vec<String>, processed_artists: &mut Vec<String>) {
    let directory_path = directory_entry.path();
    let directory_path = directory_path.to_str().expect("Directory path should have been converted to a string");

    let file_type = directory_entry.file_type().expect("File type should have been read");

    if file_type.is_dir() {
        // If it's a directory, keep searching down the directory tree
        scan_directory(database_connection, directory_path, processed_albums, processed_album_artists, processed_genres, processed_artists);
    }
    else if file_type.is_file() {
        // If it's a file, process that file
        scan_file(database_connection, directory_entry, processed_albums, processed_album_artists, processed_genres, processed_artists);
    }
}

fn scan_file(database_connection: &Connection, file: &DirEntry, processed_albums: &mut Vec<String>, processed_album_artists: &mut Vec<String>, processed_genres: &mut Vec<String>, processed_artists: &mut Vec<String>) {
    let file_path = file.path();
    let file_name = file.file_name();
    let file_name = file_name.to_str().expect("File name should exist");

    if file_name.ends_with(".flac") || file_name.ends_with(".mp3") {
        process_track(database_connection, &file_path, processed_albums, processed_album_artists, processed_genres, processed_artists);
    }
}

fn process_track(database_connection: &Connection, track_file_path: &PathBuf, processed_albums: &mut Vec<String>, processed_album_artists: &mut Vec<String>, processed_genres: &mut Vec<String>, processed_artists: &mut Vec<String>) {
    let metadata = Tag::new().read_from_path(&track_file_path).expect("Metadata should have been read for track");

    let album = metadata.album().unwrap().title;
    let album_artist = metadata.album_artist().unwrap_or_default();
    let artist = metadata.artist().unwrap_or_default();
    let genre = metadata.genre().unwrap_or_default();

    let track_to_process = TrackToProcess::new(
        &metadata.title().unwrap_or_default(),
        album,
        album_artist,
        artist,
        genre,
        &metadata.album_cover().unwrap_or(Picture::new(&[1], audiotags::MimeType::Png)),
        &track_file_path.as_path().to_str().unwrap_or_default(),
        &metadata.year().unwrap_or_default(),
        &metadata.track_number().unwrap_or_default(),
        &metadata.duration().unwrap_or_default(),
        &metadata.disc_number().unwrap_or_default(),
    );

    if !processed_genres.contains(&genre.to_string()) {
        music_database::add_genre_to_database(database_connection, &track_to_process);
        processed_genres.push(genre.to_string());
    }
    let genre_id = music_database::get_genre_id(&genre);

    if !processed_artists.contains(&artist.to_string()) {
        music_database::add_artist_to_database(database_connection, &track_to_process);
        processed_artists.push(artist.to_string());
    }
    let artist_id = music_database::get_artist_id(&artist);

    if !processed_album_artists.contains(&album_artist.to_string()) {
        music_database::add_album_artist_to_database(database_connection, &track_to_process, &genre_id);
        processed_album_artists.push(album_artist.to_string());
    }
    let album_artist_id = music_database::get_album_artist_id(&album_artist);

    let album_key = format!("{}{}", album, album_artist);
    if !processed_albums.contains(&album_key) {
        music_database::add_album_to_database(database_connection, &track_to_process, &genre_id, &album_artist_id);
        processed_albums.push(album_key);
    }
    let album_id = music_database::get_album_id(&album, &album_artist_id);

    music_database::add_track_to_database(database_connection, &track_to_process, &genre_id, &album_artist_id, &album_id, &artist_id);
}