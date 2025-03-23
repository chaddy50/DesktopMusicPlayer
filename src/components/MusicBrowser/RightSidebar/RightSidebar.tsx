import NowPlayingData from '@/dataObjects/NowPlayingData';
import { listen } from '@tauri-apps/api/event';
import { useState } from 'react';
import NowPlaying from './NowPlaying/NowPlaying';
import './RightSidebar.css';

interface RightSidebarProps {}

function RightSidebar(_props: RightSidebarProps) {
	const [nowPlayingData, setNowPlayingData] = useState<NowPlayingData>();

	listen<NowPlayingData>('now_playing_changed', (event) => {
		setNowPlayingData(event.payload);
	});

	if (nowPlayingData) {
		return (
			<div className='rightSidebar'>
				<NowPlaying
					playingTracks={nowPlayingData.playing_tracks}
					playingTrackIndex={nowPlayingData.playing_track_index}
				/>
			</div>
		);
	}
	return <div className='rightSidebar'></div>;
}

export default RightSidebar;
