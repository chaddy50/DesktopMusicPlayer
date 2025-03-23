import { formatTimeDuration } from '@/common/Utilities';
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
						<span className='trackInfoColumnTrackNumber'>
							{track.track_number}
						</span>
						<TrackInfo track={track} isPlaying={playingTrackIndex === index} />
						<span className='trackInfoColumnDuration'>
							{formatTimeDuration(track.duration_in_seconds)}
						</span>
					</div>
				);
			})}
		</div>
	);
}

export default NowPlaying;
