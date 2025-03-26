import NowPlayingStore from '@/state/NowPlayingStore';
import { invoke } from '@tauri-apps/api/core';
import { observer } from 'mobx-react';

const PlayPauseButton = observer(() => {
	const isPlaying = NowPlayingStore.isPlaying;
	return (
		<>
			{isPlaying ? (
				<button
					onClick={() => {
						invoke('on_pause_button_clicked');
					}}
				>
					Pause
				</button>
			) : (
				<button
					onClick={() => {
						invoke('on_play_button_clicked');
					}}
				>
					Play
				</button>
			)}
		</>
	);
});

export default PlayPauseButton;
