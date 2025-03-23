import TrackData from '@/dataObjects/TrackData';
import NextButton from './NextButton';
import './PlayerControls.css';
import PlayingTrack from './PlayingTrack';
import PlayPauseButton from './PlayPauseButton';
import PreviousButton from './PreviousButton';

interface PlayerControlsProps {
	isPlaying: boolean;
	playing_track: TrackData;
}

function PlayerControls(props: PlayerControlsProps) {
	const { isPlaying, playing_track } = props;

	return (
		<div className='playerControlsContainer'>
			<PlayingTrack playingTrack={playing_track} />
			<div>
				<PreviousButton />
				<PlayPauseButton isPlaying={isPlaying} />
				<NextButton />
			</div>
		</div>
	);
}

export default PlayerControls;
