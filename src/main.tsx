import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router';
import TopBar from './components/TopBar/TopBar';
import './main.css';
import MusicPlayerRoutes from './navigation/MusicPlayerRoutes';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<BrowserRouter>
			<TopBar />
			<MusicPlayerRoutes />
		</BrowserRouter>
	</React.StrictMode>
);
