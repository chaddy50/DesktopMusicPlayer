import TrackData from './TrackData';

interface NowPlayingData {
	playing_tracks: TrackData[];
	playing_track_index: number;
	is_paused: boolean;
	is_playing: boolean;
}

export default NowPlayingData;
