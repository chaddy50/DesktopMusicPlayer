import { Provider } from 'mobx-react';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router';
import TopBar from './components/TopBar/TopBar';
import './main.css';
import MusicPlayerRoutes from './navigation/MusicPlayerRoutes';
import NowPlayingStore from './state/NowPlayingStore';

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
