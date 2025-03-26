import AlbumArtistData from '@/dataObjects/AlbumArtistData';
import AlbumData from '@/dataObjects/AlbumData';
import { Dispatch, RefObject, SetStateAction } from 'react';
import AlbumBrowser from './AlbumBrowser/AlbumBrowser';
import './MainPane.css';
import TrackBrowser from './TrackBrowser/TrackBrowser';

interface MainPaneProps {
	albums: AlbumData[];
	albumArtist: AlbumArtistData;
	selectedAlbumIndex: number;
	setSelectedAlbumIndex: Dispatch<SetStateAction<number>>;
	albumListContainerRef: RefObject<HTMLDivElement> | undefined;
}

function MainPane(props: MainPaneProps) {
	const {
		albums,
		albumArtist,
		selectedAlbumIndex,
		setSelectedAlbumIndex,
		albumListContainerRef,
	} = props;

	return (
		<div data-testid='mainPaneContainer' className='mainPaneContainer'>
			<AlbumBrowser
				albumListContainerRef={albumListContainerRef}
				albums={albums}
				albumArtist={albumArtist}
				selectedAlbumIndex={selectedAlbumIndex}
				setSelectedAlbumIndex={setSelectedAlbumIndex}
			/>

			{selectedAlbumIndex > -1 && (
				<TrackBrowser album={albums[selectedAlbumIndex]} />
			)}
		</div>
	);
}

export default MainPane;
