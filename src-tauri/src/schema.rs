// @generated automatically by Diesel CLI.

diesel::table! {
    album_artists (id) {
        id -> Integer,
        name -> Text,
        genre_id -> Integer,
        sort_name -> Text,
    }
}

diesel::table! {
    albums (id) {
        id -> Integer,
        name -> Text,
        genre_id -> Integer,
        album_artist_id -> Integer,
        artwork_data -> Nullable<Text>,
        year -> Nullable<Integer>,
    }
}

diesel::table! {
    artists (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    genres (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    settings (id) {
        id -> Integer,
        key -> Text,
        value -> Text,
    }
}

diesel::table! {
    tracks (id) {
        id -> Integer,
        name -> Text,
        genre_id -> Integer,
        album_artist_id -> Integer,
        album_id -> Integer,
        artist_id -> Integer,
        track_number -> Nullable<Integer>,
        disc_number -> Nullable<Integer>,
        file_path -> Text,
        duration_in_seconds -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    album_artists,
    albums,
    artists,
    genres,
    settings,
    tracks,
);
