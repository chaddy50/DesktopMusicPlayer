import AlbumArtistData from '@/dataObjects/AlbumArtistData';
import AlbumData from '@/dataObjects/AlbumData';
import { Dispatch, RefObject, SetStateAction } from 'react';
import AlbumCard from './AlbumCard';

interface AlbumBrowserProps {
	albums: AlbumData[];
	albumArtistData: AlbumArtistData;
	selectedAlbumIndex: number;
	setSelectedAlbumIndex: Dispatch<SetStateAction<number>>;
	albumListContainerRef: RefObject<HTMLDivElement>;
}

function AlbumBrowser(props: AlbumBrowserProps) {
	const {
		albums,
		albumArtistData,
		selectedAlbumIndex,
		setSelectedAlbumIndex,
		albumListContainerRef,
	} = props;

	return (
		<div className='albumListContainer' ref={albumListContainerRef}>
			{albums.map((albumData, index) => {
				const indexToSelect = index === selectedAlbumIndex ? -1 : index;
				return (
					<AlbumCard
						key={albumData.id}
						albumData={albumData}
						albumArtistData={albumArtistData}
						isSelected={index === selectedAlbumIndex}
						selectAlbum={() => setSelectedAlbumIndex(indexToSelect)}
					/>
				);
			})}
		</div>
	);
}

export default AlbumBrowser;
