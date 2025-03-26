import NowPlayingData from '@/dataObjects/NowPlayingData';
import NowPlayingStore from '@/state/NowPlayingStore';
import { selectedGenreStore } from '@/state/SelectedGenreStore';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { observer } from 'mobx-react';
import { useEffect } from 'react';
import { useLocation, useNavigate } from 'react-router';
import PlayerControls from './PlayerControls/PlayerControls';
import PlayingTrack from './PlayingTrack';
import './TopBar.css';

interface TopBarProps {}

const TopBar = observer((_props: TopBarProps) => {
	const currentLocation = useLocation();
	const navigate = useNavigate();

	const selectedGenre = selectedGenreStore.genre;

	useEffect(() => {
		async function getNowPlayingData() {
			const nowPlayingData: NowPlayingData = await invoke(
				'refresh_now_playing_data'
			);
			NowPlayingStore.update(nowPlayingData);
		}

		getNowPlayingData();
	}, []);

	listen<NowPlayingData>('now_playing_changed', (event) => {
		NowPlayingStore.update(event.payload);
	});

	return (
		<div className='topBar'>
			<div className='backButton'>
				{currentLocation.pathname !== '/' && (
					<>
						<button onClick={() => navigate(-1)}>Go Back</button>
						<h2>{selectedGenre?.name}</h2>
					</>
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
});

export default TopBar;
