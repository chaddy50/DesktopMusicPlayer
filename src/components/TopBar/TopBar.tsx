import NowPlayingData from '@/dataObjects/NowPlayingData';
import { nowPlayingStore } from '@/state/NowPlayingStore';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect } from 'react';
import { useLocation, useNavigate } from 'react-router';
import PlayerControls from './PlayerControls/PlayerControls';
import PlayingTrack from './PlayingTrack';
import './TopBar.css';

interface TopBarProps {}

function TopBar(_props: TopBarProps) {
	const currentLocation = useLocation();
	const navigate = useNavigate();

	useEffect(() => {
		async function getNowPlayingData() {
			const nowPlayingData: NowPlayingData = await invoke(
				'refresh_now_playing_data'
			);
			nowPlayingStore.update(nowPlayingData);
		}

		getNowPlayingData();
	}, []);

	listen<NowPlayingData>('now_playing_changed', (event) => {
		nowPlayingStore.update(event.payload);
	});

	return (
		<div className='topBar'>
			<div className='backButton'>
				{currentLocation.pathname !== '/' && (
					<button onClick={() => navigate(-1)}>Go Back</button>
				)}
			</div>
			<div className='playerControls'>
				<PlayerControls />
			</div>
			<div className='playingTrack'>
				<PlayingTrack />
			</div>
		</div>
	);
}

export default TopBar;
