import NowPlayingData from '@/dataObjects/NowPlayingData';
import { action, computed, makeObservable, observable } from 'mobx';

class NowPlayingStore {
	constructor() {
		makeObservable(this, {
			data: observable,
			update: action,
			playingTrack: computed,
		});
	}

	data: NowPlayingData | null = null;

	update(newNowPlayingData: NowPlayingData) {
		this.data = newNowPlayingData;
	}

	get isPlaying() {
		return this.data?.is_playing ?? false;
	}

	get playingTrack() {
		return this.data?.playing_tracks[this.data.playing_track_index];
	}

	get playingTracks() {
		return this.data?.playing_tracks ?? [];
	}

	get playingTrackIndex() {
		return this.data?.playing_track_index ?? -1;
	}
}

export const nowPlayingStore = new NowPlayingStore();
