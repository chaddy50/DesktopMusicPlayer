import { selectedGenreStore } from '@/state/SelectedGenreStore';
import { listen } from '@tauri-apps/api/event';
import { observer } from 'mobx-react';
import { useCallback, useMemo } from 'react';
import { useLocation, useNavigate } from 'react-router';
import PlayerControls from './PlayerControls/PlayerControls';
import PlayingTrack from './PlayingTrack';
import './TopBar.css';

interface TopBarProps {}

const TopBar = observer((_props: TopBarProps) => {
	const currentLocation = useLocation();
	const navigate = useNavigate();

	listen('open_settings', () => {
		navigate('settings');
	});

	const goBack = useCallback(() => {
		console.log(currentLocation.pathname);
		if (currentLocation.pathname === '/settings') {
			navigate('/');
		} else {
			navigate(-1);
		}
	}, [currentLocation]);

	const locationHeader = useMemo(() => {
		switch (currentLocation.pathname) {
			case '/settings':
				return 'Settings';
			case '/musicBrowser':
				return selectedGenreStore.genre?.name;
			default:
				return 'Go Back';
		}
	}, [currentLocation]);

	return (
		<div className='topBar'>
			<div className='backButton'>
				{currentLocation.pathname !== '/' && (
					<>
						<button onClick={goBack}>Go Back</button>
						<h2>{locationHeader}</h2>
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
