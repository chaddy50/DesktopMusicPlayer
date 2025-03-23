import MusicBrowser from '@/components/MusicBrowser/MusicBrowser';
import { Route, Routes } from 'react-router';

function MusicPlayerRoutes() {
	return (
		<Routes>
			<Route path='/' element={<MusicBrowser />} />
		</Routes>
	);
}

export default MusicPlayerRoutes;
