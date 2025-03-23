import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router';
import MusicPlayerRoutes from './navigation/MusicPlayerRoutes';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<BrowserRouter>
			<MusicPlayerRoutes />
		</BrowserRouter>
	</React.StrictMode>
);
