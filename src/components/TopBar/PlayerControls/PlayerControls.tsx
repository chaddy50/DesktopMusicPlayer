import NextButton from './NextButton';
import './PlayerControls.css';
import PlayPauseButton from './PlayPauseButton';
import PreviousButton from './PreviousButton';

const PlayerControls = () => {
	return (
		<div className='playerControlsContainer'>
			<PreviousButton />
			<PlayPauseButton />
			<NextButton />
		</div>
	);
};

export default PlayerControls;
