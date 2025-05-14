import { listen } from '@tauri-apps/api/event';
import { Provider } from 'mobx-react';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router';
import TopBar from './components/TopBar/TopBar';
import NowPlayingData from './dataObjects/NowPlayingData';
import './main.css';
import MusicPlayerRoutes from './navigation/MusicPlayerRoutes';
import NowPlayingStore from './state/NowPlayingStore';

listen<NowPlayingData>('now_playing_changed', (event) => {
	NowPlayingStore.update(event.payload);
});

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<Provider NowPlayingStore={NowPlayingStore}>
			<BrowserRouter>
				<TopBar />
				<MusicPlayerRoutes />
			</BrowserRouter>
		</Provider>
	</React.StrictMode>
);
