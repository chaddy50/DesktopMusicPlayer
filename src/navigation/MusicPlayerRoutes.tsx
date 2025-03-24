import Home from '@/components/Home/Home';
import MusicBrowser from '@/components/MusicBrowser/MusicBrowser';
import { Route, Routes } from 'react-router';

function MusicPlayerRoutes() {
	return (
		<Routes>
			<Route path='/' element={<Home />} />
			<Route path='/musicBrowser' element={<MusicBrowser />} />
		</Routes>
	);
}

export default MusicPlayerRoutes;
