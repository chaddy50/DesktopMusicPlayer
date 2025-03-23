import NowPlayingData from '@/dataObjects/NowPlayingData';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';
import { useLocation, useNavigate } from 'react-router';
import PlayerControls from './PlayerControls/PlayerControls';
import PlayingTrack from './PlayerControls/PlayingTrack';
import './TopBar.css';

interface TopBarProps {}

function TopBar(_props: TopBarProps) {
	const currentLocation = useLocation();
	const navigate = useNavigate();

	const [nowPlayingData, setNowPlayingData] = useState<NowPlayingData>();

	useEffect(() => {
		async function getNowPlayingData() {
			const nowPlayingData: NowPlayingData = await invoke(
				'refresh_now_playing_data'
			);
			setNowPlayingData(nowPlayingData);
		}

		getNowPlayingData();
	}, [setNowPlayingData]);

	listen<NowPlayingData>('now_playing_changed', (event) => {
		setNowPlayingData(event.payload);
	});

	if (nowPlayingData) {
		return (
			<div className='topBar'>
				<div className='backButton'>
					{currentLocation.pathname !== '/' && (
						<button onClick={() => navigate(-1)}>Go Back</button>
					)}
				</div>
				<div className='playerControls'>
					<PlayerControls nowPlayingData={nowPlayingData} />
				</div>
				<div className='playingTrack'>
					<PlayingTrack
						playingTrack={
							nowPlayingData.playing_tracks[nowPlayingData.playing_track_index]
						}
					/>
				</div>
			</div>
		);
	}
}

export default TopBar;
