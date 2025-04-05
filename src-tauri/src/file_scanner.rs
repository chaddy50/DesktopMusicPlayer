use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    path::PathBuf,
};

use audiotags::{Picture, Tag};
use base64::{engine::general_purpose, Engine};
use diesel::{QueryResult, SqliteConnection};
use music_player_lib::music_database::{
    self, album::NewAlbumDatabaseObject, album_artist::NewAlbumArtist, artist::NewArtist,
    genre::NewGenre, track_to_process::TrackToProcess,
};

pub fn build_music_database() {
    if music_database::does_database_already_exist() {
        return;
    }

    let mut database_connection = music_database::open_database_connection();

    let mut processed_artists: HashMap<String, i32> = HashMap::new();
    let mut processed_albums: HashMap<String, i32> = HashMap::new();
    let mut processed_album_artists: HashMap<String, i32> = HashMap::new();
    let mut processed_genres: HashMap<String, i32> = HashMap::new();

    let music_directories = vec![
        "Video Game",
        "Rock",
        "Jazz",
        "Classic Rock",
        "Ambient",
        "Electronic",
        "Mariachi",
        "Movie",
    ];

    for directory in music_directories {
        scan_directory(
            &mut database_connection,
            format!("/home/nathan/Music/{directory}").as_str(),
            &mut processed_albums,
            &mut processed_album_artists,
            &mut processed_genres,
            &mut processed_artists,
        );
    }
}

fn scan_directory(
    database_connection: &mut SqliteConnection,
    directory_path: &str,
    processed_albums: &mut HashMap<String, i32>,
    processed_album_artists: &mut HashMap<String, i32>,
    processed_genres: &mut HashMap<String, i32>,
    processed_artists: &mut HashMap<String, i32>,
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
    database_connection: &mut SqliteConnection,
    directory_entry: &DirEntry,
    processed_albums: &mut HashMap<String, i32>,
    processed_album_artists: &mut HashMap<String, i32>,
    processed_genres: &mut HashMap<String, i32>,
    processed_artists: &mut HashMap<String, i32>,
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
    database_connection: &mut SqliteConnection,
    file: &DirEntry,
    processed_albums: &mut HashMap<String, i32>,
    processed_album_artists: &mut HashMap<String, i32>,
    processed_genres: &mut HashMap<String, i32>,
    processed_artists: &mut HashMap<String, i32>,
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
        )
        .unwrap();
    }
}

fn process_track(
    database_connection: &mut SqliteConnection,
    track_file_path: &PathBuf,
    processed_albums: &mut HashMap<String, i32>,
    processed_album_artists: &mut HashMap<String, i32>,
    processed_genres: &mut HashMap<String, i32>,
    processed_artists: &mut HashMap<String, i32>,
) -> QueryResult<()> {
    let metadata = Tag::new()
        .read_from_path(track_file_path)
        .expect("Metadata should have been read for track");

    let track_to_process = TrackToProcess::new(
        metadata.title().unwrap_or_default(),
        metadata.album().unwrap().title,
        metadata.album_artist().unwrap_or_default(),
        metadata.artist().unwrap_or_default(),
        metadata.genre().unwrap_or_default(),
        &metadata
            .album_cover()
            .unwrap_or(Picture::new(&[1], audiotags::MimeType::Png)),
        track_file_path.as_path().to_str().unwrap_or_default(),
        &metadata.year().unwrap_or_default(),
        &metadata.track_number().unwrap_or_default(),
        &metadata.duration().unwrap_or_default(),
        &metadata.disc_number().unwrap_or_default(),
    );

    let genre_id = process_genre(processed_genres, &track_to_process.genre);
    let artist_id = process_artist(processed_artists, &track_to_process.artist);
    let album_artist_id = process_album_artist(
        processed_album_artists,
        &track_to_process.album_artist,
        genre_id,
    );
    let album_id = process_album(
        processed_albums,
        &track_to_process.album,
        genre_id,
        album_artist_id,
        &track_to_process.artwork,
        track_to_process.year,
    );

    music_database::add_track_to_database(
        database_connection,
        &track_to_process,
        genre_id,
        album_artist_id,
        album_id,
        artist_id,
    );

    return Ok(());
}

fn process_genre<'a>(
    processed_genres: &'a mut HashMap<String, i32>,
    genre_name: &'a String,
) -> &'a i32 {
    if !processed_genres.contains_key(genre_name) {
        let genre_id = music_database::add_genre_to_database(NewGenre {
            name: genre_name.clone(),
        })
        .unwrap();
        processed_genres.insert(genre_name.clone(), genre_id);
    }
    processed_genres
        .get(genre_name)
        .expect("genre_id should exist")
}

fn process_artist<'a>(
    processed_artists: &'a mut HashMap<String, i32>,
    artist_name: &'a String,
) -> &'a i32 {
    if !processed_artists.contains_key(artist_name) {
        let artist_id = music_database::add_artist_to_database(NewArtist {
            name: artist_name.clone(),
        })
        .unwrap();
        processed_artists.insert(artist_name.clone(), artist_id);
    }
    processed_artists
        .get(artist_name)
        .expect("artist_id should exist")
}

fn process_album_artist<'a>(
    processed_album_artists: &'a mut HashMap<String, i32>,
    album_artist_name: &String,
    genre_id: &i32,
) -> &'a i32 {
    if !processed_album_artists.contains_key(album_artist_name) {
        let album_artist_id = music_database::add_album_artist_to_database(NewAlbumArtist {
            name: &album_artist_name,
            genre_id: &genre_id,
            sort_name: &get_sort_value_for_string(album_artist_name),
        })
        .unwrap();
        processed_album_artists.insert(album_artist_name.clone(), album_artist_id);
    }
    processed_album_artists
        .get(album_artist_name)
        .expect("album_artist_id should exist")
}

fn process_album<'a>(
    processed_albums: &'a mut HashMap<String, i32>,
    album_name: &String,
    genre_id: &'a i32,
    album_artist_id: &'a i32,
    artwork: &'a Picture,
    year: i32,
) -> &'a i32 {
    let album_key = format!("{}{}", album_name, album_artist_id);

    if !processed_albums.contains_key(&album_key) {
        let cover_data = artwork.data;
        let mime_type = artwork.mime_type;

        let mut artwork_data = "NO_ARTWORK".to_string();
        if cover_data != [1] {
            let cover_data = convert_artwork_data_to_base_64(cover_data);
            artwork_data = format!("data:image/{:?};base64,{}", mime_type, &cover_data);
        }

        let album_id = music_database::add_album_to_database(NewAlbumDatabaseObject {
            name: album_name,
            genre_id: genre_id,
            album_artist_id: album_artist_id,
            artwork_data,
            year: year,
        })
        .unwrap();
        processed_albums.insert(album_key.to_string(), album_id);
    }
    processed_albums
        .get(&album_key)
        .expect("album_id should exist")
}

fn convert_artwork_data_to_base_64(artwork_data: &[u8]) -> String {
    general_purpose::STANDARD.encode(artwork_data)
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
}
