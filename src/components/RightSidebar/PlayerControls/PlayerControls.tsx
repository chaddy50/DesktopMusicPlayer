import TrackData from '@/dataObjects/TrackData';
import NextButton from './NextButton';
import './PlayerControls.css';
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
			<p>{playing_track ? playing_track.name : 'No track'}</p>
			<div>
				<PreviousButton />
				<PlayPauseButton isPlaying={isPlaying} />
				<NextButton />
			</div>
		</div>
	);
}

export default PlayerControls;
