// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs::{self, DirEntry},
    io::Error,
    path::PathBuf,
};

use audiotags::Tag;
use sqlite::{Connection, State};

fn main() {
    build_music_database();
    music_player_lib::run();
}

fn build_music_database() {
    let database_connection = sqlite::open("music_database.db");
    match database_connection {
        Ok(database_connection) => {
            create_database_tables(&database_connection);

            // retrieve_audio_files_from_directory(&database_connection, "/home/nathan/Music/Video Game");
            // retrieve_audio_files_from_directory(&database_connection, "/home/nathan/Music/Rock");
            // retrieve_audio_files_from_directory(&database_connection, "/home/nathan/Music/Jazz");
            // retrieve_audio_files_from_directory(&database_connection, "/home/nathan/Music/Classic Rock");
            // retrieve_audio_files_from_directory(&database_connection, "/home/nathan/Music/Ambient");
            // retrieve_audio_files_from_directory(&database_connection, "/home/nathan/Music/Electronic");
        }
        Err(error) => {
            println!("Error connecting to database: {}", error);
        }
    }
}

fn create_database_tables(database_connection: &Connection) {
    let query = "
    CREATE TABLE IF NOT EXISTS genres (name TEXT PRIMARY KEY);
    CREATE TABLE IF NOT EXISTS albumArtists (name TEXT PRIMARY KEY, genre TEXT);
    CREATE TABLE IF NOT EXISTS albums (name TEXT PRIMARY KEY, genre TEXT, albumArtist TEXT);
    CREATE TABLE IF NOT EXISTS songs (name TEXT PRIMARY KEY, genre TEXT, albumArtist TEXT, album TEXT);
    ";
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
            let song = Song {
                title: metadata.title().unwrap_or_default(),
                album: metadata.album().unwrap().title,
                album_artist: metadata.album_artist().unwrap_or_default(),
                genre: metadata.genre().unwrap_or_default(),
            };

            add_song_to_database(database_connection, song);
            add_album_to_database(database_connection, song);
            add_album_artist_to_database(database_connection, song);
            add_genre_to_database(database_connection, song);
        }
        Err(error) => println!("Error: {}", error),
    }
}

fn add_song_to_database(database_connection: &Connection, song: Song) {
    if !song.title.contains('\'') {
        let query = format!(
            r#"
            INSERT OR IGNORE INTO songs VALUES ('{}', '{}', '{}', '{}');
            "#,
            song.title, song.album, song.album_artist, song.genre,
        );
        let result = database_connection.execute(query);
        match result {
            Ok(_result) => {}
            Err(_error) => {
                println!("Error adding song to database: {}", song.title);
            }
        }
    }
}

fn add_album_to_database(database_connection: &Connection, song: Song) {
    if !song.title.contains('\'') {
        let query = format!(
            r#"
            INSERT OR IGNORE INTO albums VALUES ('{}', '{}', '{}');
            "#,
            song.album, song.album_artist, song.genre,
        );
        let result = database_connection.execute(query);
        match result {
            Ok(_result) => {}
            Err(_error) => {
                println!("Error adding album to database: {}", song.album);
            }
        }
    }
}

fn add_album_artist_to_database(database_connection: &Connection, song: Song) {
    if !song.title.contains('\'') {
        let query = format!(
            r#"
            INSERT OR IGNORE INTO albumArtists VALUES ('{}', '{}');
            "#,
            song.album_artist, song.genre,
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
}

fn add_genre_to_database(database_connection: &Connection, song: Song) {
    if !song.title.contains('\'') {
        let query = format!(
            r#"
            INSERT OR IGNORE INTO genres VALUES ('{}');
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
}

#[derive(Clone, Copy)]
struct Song<'a> {
    title: &'a str,
    album: &'a str,
    album_artist: &'a str,
    genre: &'a str,
}
