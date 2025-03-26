import AlbumArtistData from '@/dataObjects/AlbumArtistData';
import '../AlbumArtistBrowser.css';

interface AlbumArtistCardProps {
	albumArtist: AlbumArtistData;
	isSelected: boolean;
	selectArtist(): void;
}

function AlbumArtistCard(props: AlbumArtistCardProps) {
	const { albumArtist, isSelected, selectArtist } = props;

	return (
		<div
			data-testid={'albumArtistCard' + albumArtist.id}
			className='artistCard'
			onClick={selectArtist}
		>
			<p className={isSelected ? 'selectedArtistCard' : 'unselectedArtistCard'}>
				{albumArtist.name}
			</p>
		</div>
	);
}

export default AlbumArtistCard;
