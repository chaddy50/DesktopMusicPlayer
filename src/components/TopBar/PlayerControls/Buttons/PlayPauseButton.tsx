import NowPlayingStore from '@/state/NowPlayingStore';
import PauseIcon from '@mui/icons-material/Pause';
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import { invoke } from '@tauri-apps/api/core';
import { observer } from 'mobx-react';
import '../PlayerControls.css';

const PlayPauseButton = observer(() => {
	const isPlaying = NowPlayingStore.isPlaying;

	return (
		<>
			{isPlaying ? (
				<div
					onClick={() => {
						invoke('on_pause_button_clicked');
					}}
				>
					<PauseIcon className='playerControlsButton' fontSize='large' />
				</div>
			) : (
				<div
					onClick={() => {
						invoke('on_play_button_clicked');
					}}
				>
					<PlayArrowIcon className='playerControlsButton' fontSize='large' />
				</div>
			)}
		</>
	);
});

export default PlayPauseButton;
