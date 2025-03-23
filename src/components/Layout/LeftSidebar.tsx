import AlbumArtistData from '@/dataObjects/AlbumArtistData';
import { Dispatch, SetStateAction } from 'react';
import AlbumArtistBrowser from '../AlbumArtistBrowser/AlbumArtistBrowser';

interface LeftSidebarProps {
	albumArtists: AlbumArtistData[];
	selectedAlbumArtistIndex: number;
	setSelectedAlbumArtistIndex: Dispatch<SetStateAction<number>>;
}

function LeftSidebar(props: LeftSidebarProps) {
	const {
		albumArtists,
		selectedAlbumArtistIndex,
		setSelectedAlbumArtistIndex,
	} = props;

	return (
		<div className='leftSideBar'>
			<AlbumArtistBrowser
				albumArtists={albumArtists}
				selectedAlbumArtistIndex={selectedAlbumArtistIndex}
				setSelectedAlbumArtistIndex={setSelectedAlbumArtistIndex}
			/>
		</div>
	);
}

export default LeftSidebar;
