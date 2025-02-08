use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    path::PathBuf,
};

use audiotags::{Picture, Tag};
use music_player_lib::music_database::{self, track_to_process::TrackToProcess};
use sqlite::Connection;

pub fn build_music_database() {
    if music_database::does_database_already_exist() {
        return;
    }

    let database_connection = music_database::open_database_connection();

    music_database::create_tables(&database_connection);

    let mut processed_artists: HashMap<String, i64> = HashMap::new();
    let mut processed_albums: HashMap<String, i64> = HashMap::new();
    let mut processed_album_artists: HashMap<String, i64> = HashMap::new();
    let mut processed_genres: HashMap<String, i64> = HashMap::new();

    scan_directory(
        &database_connection,
        "/home/nathan/Music/Video Game",
        &mut processed_albums,
        &mut processed_album_artists,
        &mut processed_genres,
        &mut processed_artists,
    );
    scan_directory(
        &database_connection,
        "/home/nathan/Music/Rock",
        &mut processed_albums,
        &mut processed_album_artists,
        &mut processed_genres,
        &mut processed_artists,
    );
    scan_directory(
        &database_connection,
        "/home/nathan/Music/Jazz",
        &mut processed_albums,
        &mut processed_album_artists,
        &mut processed_genres,
        &mut processed_artists,
    );
    scan_directory(
        &database_connection,
        "/home/nathan/Music/Classic Rock",
        &mut processed_albums,
        &mut processed_album_artists,
        &mut processed_genres,
        &mut processed_artists,
    );
    scan_directory(
        &database_connection,
        "/home/nathan/Music/Ambient",
        &mut processed_albums,
        &mut processed_album_artists,
        &mut processed_genres,
        &mut processed_artists,
    );
    scan_directory(
        &database_connection,
        "/home/nathan/Music/Electronic",
        &mut processed_albums,
        &mut processed_album_artists,
        &mut processed_genres,
        &mut processed_artists,
    );
}

fn scan_directory(
    database_connection: &Connection,
    directory_path: &str,
    processed_albums: &mut HashMap<String, i64>,
    processed_album_artists: &mut HashMap<String, i64>,
    processed_genres: &mut HashMap<String, i64>,
    processed_artists: &mut HashMap<String, i64>,
) {
    let directory_entries = fs::read_dir(directory_path).unwrap_or_else(|_| {
        panic!(
            "Directory entries should have been read: {}",
            directory_path
        )
    });

    for directory_entry_result in directory_entries {
        let directory_entry =
            directory_entry_result.expect("Directory entry should have been read");
        scan_directory_entry(
            database_connection,
            &directory_entry,
            processed_albums,
            processed_album_artists,
            processed_genres,
            processed_artists,
        );
    }
}

fn scan_directory_entry(
    database_connection: &Connection,
    directory_entry: &DirEntry,
    processed_albums: &mut HashMap<String, i64>,
    processed_album_artists: &mut HashMap<String, i64>,
    processed_genres: &mut HashMap<String, i64>,
    processed_artists: &mut HashMap<String, i64>,
) {
    let directory_path = directory_entry.path();
    let directory_path = directory_path
        .to_str()
        .expect("Directory path should have been converted to a string");

    let file_type = directory_entry
        .file_type()
        .expect("File type should have been read");

    if file_type.is_dir() {
        // If it's a directory, keep searching down the directory tree
        scan_directory(
            database_connection,
            directory_path,
            processed_albums,
            processed_album_artists,
            processed_genres,
            processed_artists,
        );
    } else if file_type.is_file() {
        // If it's a file, process that file
        scan_file(
            database_connection,
            directory_entry,
            processed_albums,
            processed_album_artists,
            processed_genres,
            processed_artists,
        );
    }
}

fn scan_file(
    database_connection: &Connection,
    file: &DirEntry,
    processed_albums: &mut HashMap<String, i64>,
    processed_album_artists: &mut HashMap<String, i64>,
    processed_genres: &mut HashMap<String, i64>,
    processed_artists: &mut HashMap<String, i64>,
) {
    let file_path = file.path();
    let file_name = file.file_name();
    let file_name = file_name.to_str().expect("File name should exist");

    if file_name.ends_with(".flac") || file_name.ends_with(".mp3") {
        process_track(
            database_connection,
            &file_path,
            processed_albums,
            processed_album_artists,
            processed_genres,
            processed_artists,
        );
    }
}

fn process_track(
    database_connection: &Connection,
    track_file_path: &PathBuf,
    processed_albums: &mut HashMap<String, i64>,
    processed_album_artists: &mut HashMap<String, i64>,
    processed_genres: &mut HashMap<String, i64>,
    processed_artists: &mut HashMap<String, i64>,
) {
    let metadata = Tag::new()
        .read_from_path(track_file_path)
        .expect("Metadata should have been read for track");

    let album = metadata.album().unwrap().title;
    let album_artist = metadata.album_artist().unwrap_or_default();
    let artist = metadata.artist().unwrap_or_default();
    let genre = metadata.genre().unwrap_or_default();

    let track_to_process = TrackToProcess::new(
        metadata.title().unwrap_or_default(),
        album,
        album_artist,
        artist,
        genre,
        &metadata
            .album_cover()
            .unwrap_or(Picture::new(&[1], audiotags::MimeType::Png)),
        track_file_path.as_path().to_str().unwrap_or_default(),
        &metadata.year().unwrap_or_default(),
        &metadata.track_number().unwrap_or_default(),
        &metadata.duration().unwrap_or_default(),
        &metadata.disc_number().unwrap_or_default(),
    );

    processed_genres
        .entry(genre.to_string())
        .or_insert_with(|| {
            music_database::add_genre_to_database(database_connection, &track_to_process)
        });
    let genre_id = processed_genres.get(genre).expect("genre_id should exist");

    processed_artists
        .entry(artist.to_string())
        .or_insert_with(|| {
            music_database::add_artist_to_database(database_connection, &track_to_process)
        });
    let artist_id = processed_artists
        .get(artist)
        .expect("artist_id should exist");

    processed_album_artists
        .entry(album_artist.to_string())
        .or_insert_with(|| {
            music_database::add_album_artist_to_database(
                database_connection,
                &track_to_process,
                genre_id,
            )
        });
    let album_artist_id = processed_album_artists
        .get(album_artist)
        .expect("album_artist_id should exist");

    let album_key = format!("{}{}", album, album_artist);
    if !processed_albums.contains_key(&album_key) {
        let album_id = music_database::add_album_to_database(
            database_connection,
            &track_to_process,
            genre_id,
            album_artist_id,
        );
        processed_albums.insert(album_key.to_string(), album_id);
    }
    let album_id = processed_albums
        .get(&album_key)
        .expect("album_id should exist");

    music_database::add_track_to_database(
        database_connection,
        &track_to_process,
        genre_id,
        album_artist_id,
        album_id,
        artist_id,
    );
}
