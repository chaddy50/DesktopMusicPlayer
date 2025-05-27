import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { Provider } from 'mobx-react';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router';
import RightSidebar from './components/RightSidebar/RightSidebar';
import TopBar from './components/TopBar/TopBar';
import NowPlayingData from './dataObjects/NowPlayingData';
import SettingData from './dataObjects/SettingData';
import './main.css';
import MusicPlayerRoutes from './navigation/MusicPlayerRoutes';
import NowPlayingStore from './state/NowPlayingStore';
import SettingsStore from './state/SettingsStore';

listen<NowPlayingData>('now_playing_changed', (event) => {
	NowPlayingStore.update(event.payload);
});

listen<SettingData[]>('settings_changed', (event) => {
	SettingsStore.update(event.payload);
});

listen<string>('theme_changed', (event) => {
	console.log('theme: ' + event.payload);
	document
		.getElementById('root')
		?.setAttribute('data-theme', event.payload ?? 'light');
});

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<Provider NowPlayingStore={NowPlayingStore} SettingsStore={SettingsStore}>
			<BrowserRouter>
				<TopBar />
				<div className='mainPaneContainer'>
					<div className='mainContentContainer'>
						<MusicPlayerRoutes />
					</div>
					<div className='rightSidebarContainer'>
						<RightSidebar />
					</div>
				</div>
			</BrowserRouter>
		</Provider>
	</React.StrictMode>
);

invoke('update_theme');
