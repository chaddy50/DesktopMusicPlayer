import TrackData from '@/dataObjects/TrackData';
import TrackInfo from '../../../common/components/TrackInfo/TrackInfo';
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
