import NextButton from './Buttons/NextButton';
import PlayPauseButton from './Buttons/PlayPauseButton';
import PreviousButton from './Buttons/PreviousButton';
import './PlayerControls.css';

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
