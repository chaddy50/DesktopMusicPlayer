use std::{fs::{self, DirEntry}, io::Error, path::Path};

use crate::music_database::{DATABASE_PATH_MUSIC, create_tables, process_song};
use sqlite::Connection;

#[allow(dead_code)]
pub fn build_music_database() {
    if Path::new(DATABASE_PATH_MUSIC).exists() {
        return;
    }

    let database_connection = sqlite::open(DATABASE_PATH_MUSIC);
    match database_connection {
        Ok(database_connection) => {
            create_tables(&database_connection);

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
                    if file_name.ends_with(".flac") || file_name.ends_with(".mp3") {
                        process_song(database_connection, file_path);
                    }
                }
                None => println!("No file name"),
            }
        }
        Err(error) => println!("There was an error reading a file: {}", error),
    }
}