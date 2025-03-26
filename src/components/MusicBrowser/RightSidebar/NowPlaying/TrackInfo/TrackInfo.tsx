import { formatTimeDuration } from '@/common/Utilities';
import TrackData from '@/dataObjects/TrackData';
import './TrackInfo.css';

interface TrackInfoProps {
	track: TrackData;
	isPlaying: boolean;
	imageSize?: string;
}

function TrackInfo(props: TrackInfoProps) {
	const { track, isPlaying } = props;

	let containerClassName = 'trackInfoContainer';
	if (isPlaying) {
		containerClassName += ' playingTrack';
	}

	return (
		<>
			<div
				data-testid={'trackInfo' + track.file_path}
				className={containerClassName}
			>
				<span className='trackInfoColumn'>{track.name}</span>
				<span className='trackInfoColumn'>{track.artist_name}</span>
				<span className='trackInfoColumn'>{track.album_name}</span>
			</div>
			<span className='trackInfoColumnDuration'>
				{formatTimeDuration(track.duration_in_seconds)}
			</span>
		</>
	);
}

export default TrackInfo;
