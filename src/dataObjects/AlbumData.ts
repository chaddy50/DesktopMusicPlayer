import TrackData from "./TrackData";

interface AlbumData {
    id: number;
    name: string;
    genre_id: number;
    genre_name: string;
    album_artist_id: number;
    album_artist_name: string;
    artwork_source: string;
    tracks: TrackData[];
    year: number;
    duration_in_seconds: number;
}

export default AlbumData;
