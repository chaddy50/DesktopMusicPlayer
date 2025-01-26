use audiotags::Picture;

#[derive(Clone)]
pub struct TrackToProcess<'a> {
    pub title: String,
    pub album: String,
    pub album_artist: String,
    pub artist: String,
    pub genre: String,
    pub artwork: Picture<'a>,
    pub file_path: String,
    pub year: i32,
    pub track_number: u16,
    pub duration: f64,
    pub disc_number: u16,
}

impl<'a> TrackToProcess<'a> {
    pub fn new(title: &str, album: &str, album_artist: &str, artist: &str, genre: &str, artwork: &Picture<'a>, file_path: &str, year: &i32, track_number: &u16, duration: &f64, disc_number: &u16) -> TrackToProcess<'a> {
        TrackToProcess {
            title: super::escape_string_for_sql(title),
            album: super::escape_string_for_sql(album),
            album_artist: super::escape_string_for_sql(album_artist),
            artist: super::escape_string_for_sql(artist),
            genre: super::escape_string_for_sql(genre),
            artwork: artwork.clone(),
            file_path: super::escape_string_for_sql(file_path),
            year: year.clone(),
            track_number: track_number.clone(),
            duration: duration.clone(),
            disc_number: disc_number.clone(),
        }
    }
}