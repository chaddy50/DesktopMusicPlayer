import AlbumArtistData from '@/dataObjects/AlbumArtistData';
import AlbumData from '@/dataObjects/AlbumData';
import { Dispatch, RefObject, SetStateAction } from 'react';
import AlbumBrowser from '../AlbumBrowser/AlbumBrowser';
import TrackBrowser from '../TrackBrowser/TrackBrowser';

interface MainPaneProps {
	albums: AlbumData[];
	albumArtistData: AlbumArtistData;
	selectedAlbumIndex: number;
	setSelectedAlbumIndex: Dispatch<SetStateAction<number>>;
	albumListContainerRef: RefObject<HTMLDivElement>;
}

function MainPane(props: MainPaneProps) {
	const {
		albums,
		albumArtistData,
		selectedAlbumIndex,
		setSelectedAlbumIndex,
		albumListContainerRef,
	} = props;

	return (
		<div className='mainPaneContainer'>
			<AlbumBrowser
				albumListContainerRef={albumListContainerRef}
				albums={albums}
				albumArtistData={albumArtistData}
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
