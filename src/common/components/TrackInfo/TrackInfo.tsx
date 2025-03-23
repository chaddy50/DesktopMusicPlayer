import TrackData from '@/dataObjects/TrackData';
import './TrackInfo.css';

interface TrackInfoProps {
	track: TrackData;
	isPlaying: boolean;
}

function TrackInfo(props: TrackInfoProps) {
	const { track, isPlaying } = props;

	let containerClassName = 'trackInfoContainer';
	if (isPlaying) {
		containerClassName += ' playingTrack';
	}

	return (
		<div className={containerClassName}>
			<span className='trackInfoColumn'>{track.name}</span>
			<span className='trackInfoColumn'>{track.artist_name}</span>
			<span className='trackInfoColumn'>{track.album_name}</span>
		</div>
	);
}

export default TrackInfo;
