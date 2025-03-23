import { formatTimeDuration } from '@/common/Utilities';
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
					<div
						className='nowPlayingTrackContainer'
						style={
							playingTrackIndex === index
								? {
										fontWeight: 'bold',
								  }
								: {}
						}
					>
						<span className='trackInfoColumnTrackNumber'>{index + 1}</span>
						<div
							style={{
								display: 'flex',
								flex: 1,
								flexDirection: 'column',
								alignItems: 'center',
							}}
						>
							<span className='trackInfoColumn'>{track.name}</span>
							<span className='trackInfoColumn'>{track.artist_name}</span>
							<span className='trackInfoColumn'>{track.album_name}</span>
						</div>
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
