import NowPlayingData from '@/dataObjects/NowPlayingData';
import NextButton from './NextButton';
import './PlayerControls.css';
import PlayPauseButton from './PlayPauseButton';
import PreviousButton from './PreviousButton';

interface PlayerControlsProps {
	nowPlayingData: NowPlayingData;
}

function PlayerControls(props: PlayerControlsProps) {
	const { nowPlayingData } = props;

	if (nowPlayingData) {
		return (
			<div className='playerControlsContainer'>
				<PreviousButton />
				<PlayPauseButton isPlaying={nowPlayingData.is_playing} />
				<NextButton />
			</div>
		);
	}
}

export default PlayerControls;
