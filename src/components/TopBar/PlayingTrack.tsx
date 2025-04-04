import NowPlayingStore from '@/state/NowPlayingStore';
import { observer } from 'mobx-react';

const PlayingTrack = observer(() => {
	const playingTrack = NowPlayingStore.playingTrack;

	if (playingTrack) {
		return <div className='playingTrackContainer'>{playingTrack.name}</div>;
	} else {
		return <p>No track</p>;
	}
});

export default PlayingTrack;
