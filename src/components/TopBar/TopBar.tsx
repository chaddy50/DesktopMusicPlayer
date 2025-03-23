import NowPlayingData from '@/dataObjects/NowPlayingData';
import { listen } from '@tauri-apps/api/event';
import { useState } from 'react';
import { useLocation, useNavigate } from 'react-router';
import PlayerControls from './PlayerControls/PlayerControls';
import PlayingTrack from './PlayerControls/PlayingTrack';
import './TopBar.css';

interface TopBarProps {}

function TopBar(_props: TopBarProps) {
	const currentLocation = useLocation();
	const navigate = useNavigate();

	const [nowPlayingData, setNowPlayingData] = useState<NowPlayingData>();

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
