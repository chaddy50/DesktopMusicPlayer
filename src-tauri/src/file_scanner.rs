use std::fs::{self, DirEntry};

use crate::music_database;
use sqlite::Connection;

#[allow(dead_code)]
pub fn build_music_database() {
    if music_database::does_database_already_exist() {
        return;
    }

    let database_connection = music_database::open_database_connection();

    music_database::create_tables(&database_connection);

    scan_directory(&database_connection, "/home/nathan/Music/Video Game");
    scan_directory(&database_connection, "/home/nathan/Music/Rock");
    scan_directory(&database_connection, "/home/nathan/Music/Jazz");
    scan_directory(&database_connection, "/home/nathan/Music/Classic Rock");
    scan_directory(&database_connection, "/home/nathan/Music/Ambient");
    scan_directory(&database_connection, "/home/nathan/Music/Electronic");
}

fn scan_directory(database_connection: &Connection, directory_path: &str) {

    let directory_entries = fs::read_dir(directory_path).expect(format!("Directory entries should have been read: {}", directory_path).as_str());
    
    for directory_entry_result in directory_entries {
        let directory_entry = directory_entry_result.expect("Directory entry should have been read");
        scan_directory_entry(database_connection, &directory_entry);
    }
}

fn scan_directory_entry(database_connection: &Connection, directory_entry: &DirEntry) {
    let directory_path = directory_entry.path();
    let directory_path = directory_path.to_str().expect("Directory path should have been converted to a string");

    let file_type = directory_entry.file_type().expect("File type should have been read");

    if file_type.is_dir() {
        // If it's a directory, keep searching down the directory tree
        scan_directory(database_connection, directory_path);
    }
    else if file_type.is_file() {
        // If it's a file, process that file
        scan_file(database_connection, directory_entry);
    }
}

fn scan_file(database_connection: &Connection, file: &DirEntry) {
    let file_path = file.path();
    let file_name = file.file_name();
    let file_name = file_name.to_str().expect("File name should exist");

    if file_name.ends_with(".flac") || file_name.ends_with(".mp3") {
        music_database::process_track(database_connection, &file_path);
    }
}