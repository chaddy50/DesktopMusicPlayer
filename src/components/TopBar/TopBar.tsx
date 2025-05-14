import { selectedGenreStore } from '@/state/SelectedGenreStore';
import { observer } from 'mobx-react';
import { useLocation, useNavigate } from 'react-router';
import PlayerControls from './PlayerControls/PlayerControls';
import PlayingTrack from './PlayingTrack';
import './TopBar.css';

interface TopBarProps {}

const TopBar = observer((_props: TopBarProps) => {
	const currentLocation = useLocation();
	const navigate = useNavigate();

	return (
		<div className='topBar'>
			<div className='backButton'>
				{currentLocation.pathname !== '/' && (
					<>
						<button onClick={() => navigate(-1)}>Go Back</button>
						<h2>{selectedGenreStore.genre?.name}</h2>
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
