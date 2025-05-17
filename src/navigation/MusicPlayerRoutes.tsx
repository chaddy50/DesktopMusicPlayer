import Home from '@/components/Home/Home';
import MusicBrowser from '@/components/MusicBrowser/MusicBrowser';
import Settings from '@/components/Settings/Settings';
import { Route, Routes } from 'react-router';

function MusicPlayerRoutes() {
	return (
		<Routes>
			<Route path='/' element={<Home />} />
			<Route path='/musicBrowser' element={<MusicBrowser />} />
			<Route path='/settings' element={<Settings />} />
		</Routes>
	);
}

export default MusicPlayerRoutes;
