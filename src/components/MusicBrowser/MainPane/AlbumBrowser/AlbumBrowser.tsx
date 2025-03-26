import AlbumArtistData from '@/dataObjects/AlbumArtistData';
import AlbumData from '@/dataObjects/AlbumData';
import { Dispatch, RefObject, SetStateAction } from 'react';
import AlbumCard from './AlbumCard/AlbumCard';

interface AlbumBrowserProps {
	albums: AlbumData[];
	albumArtist: AlbumArtistData;
	selectedAlbumIndex: number;
	setSelectedAlbumIndex: Dispatch<SetStateAction<number>>;
	albumListContainerRef: RefObject<HTMLDivElement> | undefined;
}

function AlbumBrowser(props: AlbumBrowserProps) {
	const {
		albums,
		albumArtist,
		selectedAlbumIndex,
		setSelectedAlbumIndex,
		albumListContainerRef,
	} = props;

	return (
		<div
			data-testid='albumBrowser'
			className='albumListContainer'
			ref={albumListContainerRef}
		>
			{albums.map((albumData, index) => {
				const indexToSelect = index === selectedAlbumIndex ? -1 : index;
				return (
					<AlbumCard
						key={albumData.id}
						album={albumData}
						albumArtist={albumArtist}
						isSelected={index === selectedAlbumIndex}
						selectAlbum={() => setSelectedAlbumIndex(indexToSelect)}
					/>
				);
			})}
		</div>
	);
}

export default AlbumBrowser;
