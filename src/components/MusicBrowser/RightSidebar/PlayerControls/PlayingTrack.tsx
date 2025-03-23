import TrackInfo from '@/components/Common/TrackInfo/TrackInfo';
import TrackData from '@/dataObjects/TrackData';

interface PlayingTrackProps {
	playingTrack: TrackData;
}

function PlayingTrack(props: PlayingTrackProps) {
	const { playingTrack } = props;

	if (playingTrack) {
		return (
			<div className='playingTrackContainer'>
				<TrackInfo track={playingTrack} isPlaying />
			</div>
		);
	} else {
		return <p>No track</p>;
	}
}

export default PlayingTrack;
