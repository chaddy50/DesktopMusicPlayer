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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        title: &str,
        album: &str,
        album_artist: &str,
        artist: &str,
        genre: &str,
        artwork: &Picture<'a>,
        file_path: &str,
        year: &i32,
        track_number: &u16,
        duration: &f64,
        disc_number: &u16,
    ) -> TrackToProcess<'a> {
        TrackToProcess {
            title: title.to_string(),
            album: album.to_string(),
            album_artist: album_artist.to_string(),
            artist: artist.to_string(),
            genre: genre.to_string(),
            artwork: artwork.clone(),
            file_path: file_path.to_string(),
            year: *year,
            track_number: *track_number,
            duration: *duration,
            disc_number: *disc_number,
        }
    }
}
