import TrackData from '@/dataObjects/TrackData';
import TrackInfo from '../../../common/components/TrackInfo/TrackInfo';

interface PlayingTrackProps {
	playingTrack: TrackData;
}

function PlayingTrack(props: PlayingTrackProps) {
	const { playingTrack } = props;

	if (playingTrack) {
		return (
			<div>
				<TrackInfo track={playingTrack} isPlaying />
			</div>
		);
	} else {
		return <p>No track</p>;
	}
}

export default PlayingTrack;
