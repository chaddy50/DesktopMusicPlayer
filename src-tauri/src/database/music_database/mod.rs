use std::collections::HashMap;

use album::{Album, AlbumDatabaseObject, NewAlbumDatabaseObject};
use album_artist::{AlbumArtist, NewAlbumArtist};
use artist::{Artist, NewArtist};
use diesel::{
    dsl::count_star, prelude::Queryable, ExpressionMethods, JoinOnDsl, NullableExpressionMethods,
    QueryDsl, QueryResult, RunQueryDsl, SqliteConnection,
};
use genre::{Genre, NewGenre};
use track::{NewTrackDatabaseObject, Track};
use track_to_process::TrackToProcess;

use crate::schema::{
    album_artists, albums, artists, genres,
    tracks::{self},
};

use crate::database;

use super::{file_scanner, settings_database};

pub mod album;
pub mod album_artist;
pub mod artist;
pub mod genre;
pub mod track;
pub mod track_to_process;

pub fn rebuild() {
    delete();
    build();
}

pub fn build() {
    if does_database_already_exist() {
        return;
    }

    let mut database_connection = database::open_database_connection();

    let mut processed_artists: HashMap<String, i32> = HashMap::new();
    let mut processed_albums: HashMap<String, i32> = HashMap::new();
    let mut processed_album_artists: HashMap<String, i32> = HashMap::new();
    let mut processed_genres: HashMap<String, i32> = HashMap::new();

    let directories = settings_database::get_directories();
    for directory_path in directories {
        file_scanner::scan_directory(
            &mut database_connection,
            directory_path.as_str(),
            &mut processed_albums,
            &mut processed_album_artists,
            &mut processed_genres,
            &mut processed_artists,
        );
    }
}

fn delete() {
    let mut database_connection = database::open_database_connection();

    diesel::delete(album_artists::dsl::album_artists)
        .execute(&mut database_connection)
        .unwrap();

    diesel::delete(albums::dsl::albums)
        .execute(&mut database_connection)
        .unwrap();

    diesel::delete(artists::dsl::artists)
        .execute(&mut database_connection)
        .unwrap();

    diesel::delete(genres::dsl::genres)
        .execute(&mut database_connection)
        .unwrap();

    diesel::delete(tracks::dsl::tracks)
        .execute(&mut database_connection)
        .unwrap();
}

fn does_database_already_exist() -> bool {
    let mut database_connection = database::open_database_connection();

    let number_of_rows: i64 = tracks::table
        .select(count_star())
        .first(&mut database_connection)
        .unwrap();

    number_of_rows > 0
}

pub fn add_track_to_database(
    database_connection: &mut SqliteConnection,
    track_to_process: &TrackToProcess,
    genre_id: &i32,
    album_artist_id: &i32,
    album_id: &i32,
    artist_id: &i32,
) {
    let new_track = NewTrackDatabaseObject {
        name: track_to_process.title.clone(),
        genre_id,
        album_artist_id,
        album_id,
        artist_id,
        track_number: track_to_process.track_number as i32,
        disc_number: track_to_process.disc_number as i32,
        file_path: track_to_process.file_path.clone(),
        duration_in_seconds: track_to_process.duration as i32,
    };

    diesel::insert_or_ignore_into(tracks::table)
        .values(&new_track)
        .execute(database_connection)
        .unwrap();
}

pub fn add_genre_to_database(new_genre: NewGenre) -> QueryResult<i32> {
    let mut database_connection = database::open_database_connection();

    let genre: Genre = diesel::insert_or_ignore_into(genres::table)
        .values(new_genre)
        .get_result(&mut database_connection)?;

    Ok(genre.id)
}

pub fn add_artist_to_database(new_artist: NewArtist) -> QueryResult<i32> {
    let mut database_connection = database::open_database_connection();

    let artist: Artist = diesel::insert_or_ignore_into(artists::table)
        .values(new_artist)
        .get_result(&mut database_connection)?;

    Ok(artist.id)
}

pub fn add_album_artist_to_database(new_album_artist: NewAlbumArtist) -> QueryResult<i32> {
    let mut database_connection = database::open_database_connection();

    let album_artist: AlbumArtist = diesel::insert_or_ignore_into(album_artists::table)
        .values(new_album_artist)
        .get_result(&mut database_connection)?;

    Ok(album_artist.id)
}

pub fn add_album_to_database(new_album: NewAlbumDatabaseObject) -> QueryResult<i32> {
    let mut database_connection = database::open_database_connection();

    let album: AlbumDatabaseObject = diesel::insert_or_ignore_into(albums::table)
        .values(new_album)
        .get_result(&mut database_connection)?;

    Ok(album.id)
}

pub fn get_genres() -> Vec<Genre> {
    let mut database_connection = database::open_database_connection();

    genres::dsl::genres
        .select((genres::id, genres::name))
        .order_by(genres::name)
        .load::<Genre>(&mut database_connection)
        .unwrap()
}

pub fn get_album_artists_for_genre(genre_id: &i32) -> Vec<AlbumArtist> {
    let mut database_connection = database::open_database_connection();

    let mut album_artists = album_artists::dsl::album_artists
        .inner_join(albums::dsl::albums.on(albums::album_artist_id.eq(album_artists::id)))
        .inner_join(genres::dsl::genres.on(album_artists::genre_id.eq(genres::id)))
        .select((
            album_artists::id,
            album_artists::name,
            album_artists::genre_id,
            album_artists::sort_name,
        ))
        .distinct()
        .filter(albums::genre_id.eq(genre_id))
        .order_by(album_artists::sort_name)
        .load::<AlbumArtist>(&mut database_connection)
        .unwrap();

    let genre_name: String = genres::dsl::genres
        .select(genres::name)
        .filter(genres::id.eq(genre_id))
        .first(&mut database_connection)
        .unwrap();
    let all_artists_name = get_all_artists_name(&genre_name.as_str());

    album_artists.insert(
        0,
        AlbumArtist {
            id: 0,
            name: all_artists_name.to_string(),
            genre_id: genre_id.clone(),
            sort_name: all_artists_name.to_string(),
        },
    );

    album_artists
}

#[derive(Queryable)]
struct AlbumToProcess {
    id: i32,
    name: String,
    album_artist_id: i32,
    album_artist_name: String,
    genre_id: i32,
    genre_name: String,
    artwork_data: Option<String>,
    year: Option<i32>,
}

pub fn get_albums_for_album_artist(album_artist_id: &i32, genre_id: &i32) -> Vec<Album> {
    let mut database_connection = database::open_database_connection();

    let albums_to_process: Vec<AlbumToProcess>;
    if *album_artist_id != 0 {
        albums_to_process = albums::dsl::albums
            .inner_join(album_artists::table.on(albums::album_artist_id.eq(album_artists::id)))
            .inner_join(genres::table.on(genres::id.eq(albums::genre_id)))
            .select((
                albums::id,
                albums::name,
                albums::album_artist_id,
                album_artists::name,
                albums::genre_id,
                genres::name,
                albums::artwork_data,
                albums::year,
            ))
            .filter(albums::album_artist_id.eq(album_artist_id))
            .filter(albums::genre_id.eq(genre_id))
            .order_by((albums::year, albums::name))
            .load(&mut database_connection)
            .unwrap();
    } else {
        albums_to_process = albums::dsl::albums
            .inner_join(genres::table.on(albums::genre_id.eq(genres::id)))
            .inner_join(album_artists::table.on(albums::album_artist_id.eq(album_artists::id)))
            .select((
                albums::id,
                albums::name,
                albums::album_artist_id,
                album_artists::name,
                albums::genre_id,
                genres::name,
                albums::artwork_data,
                albums::year,
            ))
            .filter(albums::genre_id.eq(genre_id))
            .order_by((album_artists::sort_name, albums::year, albums::name))
            .load(&mut database_connection)
            .unwrap();
    }

    let mut albums = Vec::new();
    for album_to_process in albums_to_process {
        let tracks = get_tracks_for_album(&album_to_process.id);
        let mut duration = 0;
        for track in &tracks {
            duration += track.duration_in_seconds
        }

        albums.push(Album::new(
            album_to_process.id,
            album_to_process.name,
            album_to_process.album_artist_id,
            album_to_process.album_artist_name,
            album_to_process.genre_id,
            album_to_process.genre_name,
            album_to_process.artwork_data.unwrap_or("".to_string()),
            album_to_process.year.unwrap_or(1800),
            tracks,
            duration,
        ));
    }
    albums
}

fn get_tracks_for_album(album_id: &i32) -> Vec<Track> {
    let mut database_connection = database::open_database_connection();

    let tracks: Vec<Track> = tracks::dsl::tracks
        .inner_join(album_artists::table.on(tracks::album_artist_id.eq(album_artists::id)))
        .inner_join(artists::table.on(tracks::artist_id.eq(artists::id)))
        .inner_join(genres::table.on(tracks::genre_id.eq(genres::id)))
        .inner_join(albums::table.on(tracks::album_id.eq(albums::id)))
        .select((
            tracks::name,
            tracks::album_artist_id,
            album_artists::name,
            tracks::artist_id,
            artists::name,
            tracks::genre_id,
            genres::name,
            tracks::file_path,
            tracks::track_number.assume_not_null(),
            tracks::disc_number.assume_not_null(),
            tracks::duration_in_seconds,
            albums::name,
        ))
        .order_by((tracks::disc_number, tracks::track_number))
        .filter(tracks::album_id.eq(album_id))
        .load(&mut database_connection)
        .unwrap();

    tracks
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
