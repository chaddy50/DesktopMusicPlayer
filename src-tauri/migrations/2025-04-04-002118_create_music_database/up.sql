CREATE TABLE genres (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE album_artists (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    genre_id INTEGER NOT NULL,
    sort_name TEXT NOT NULL
);

CREATE TABLE artists (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE albums (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    genre_id INTEGER NOT NULL,
    album_artist_id INTEGER NOT NULL,
    artwork_data TEXT,
    year INTEGER
);

CREATE TABLE tracks (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    genre_id INTEGER NOT NULL,
    album_artist_id INTEGER NOT NULL,
    album_id INTEGER NOT NULL,
    artist_id INTEGER NOT NULL,
    track_number INTEGER,
    disc_number INTEGER,
    file_path TEXT NOT NULL,
    duration_in_seconds INTEGER NOT NULL
);