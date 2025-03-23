import TrackInfo from '@/components/MusicBrowser/RightSidebar/NowPlaying/TrackInfo/TrackInfo';
import TrackData from '@/dataObjects/TrackData';
import './NowPlaying.css';

interface NowPlayingProps {
	playingTracks: TrackData[];
	playingTrackIndex: number;
}

function NowPlaying(props: NowPlayingProps) {
	const { playingTracks, playingTrackIndex } = props;

	return (
		<div className='nowPlayingTrackListContainer'>
			{playingTracks?.map((track, index) => {
				return (
					<div className='nowPlayingTrackContainer'>
						<TrackInfo track={track} isPlaying={playingTrackIndex === index} />
					</div>
				);
			})}
		</div>
	);
}

export default NowPlaying;
