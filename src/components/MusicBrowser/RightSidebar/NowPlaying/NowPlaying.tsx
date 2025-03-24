import TrackInfo from '@/components/MusicBrowser/RightSidebar/NowPlaying/TrackInfo/TrackInfo';
import { nowPlayingStore } from '@/state/NowPlayingStore';
import { observer } from 'mobx-react';
import './NowPlaying.css';

const NowPlaying = observer(() => {
	const playingTracks = nowPlayingStore.playingTracks;
	const playingTrackIndex = nowPlayingStore.playingTrackIndex;

	return (
		<div className='nowPlayingTrackListContainer'>
			{playingTracks?.map((track, index) => {
				return (
					<div className='nowPlayingTrackContainer'>
						<TrackInfo
							key={track.file_path}
							track={track}
							isPlaying={playingTrackIndex === index}
						/>
					</div>
				);
			})}
		</div>
	);
});

export default NowPlaying;
