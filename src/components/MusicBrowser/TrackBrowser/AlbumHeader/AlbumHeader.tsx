import { useAlbumArtwork } from '@/common/Hooks';
import { formatTimeDuration } from '@/common/Utilities';
import '@/components/MusicBrowser/TrackBrowser/TrackBrowser.css';
import AlbumData from '@/dataObjects/AlbumData';

interface AlbumHeaderProps {
	albumData: AlbumData;
}

function AlbumHeader(props: AlbumHeaderProps) {
	const { albumData } = props;
	const imageSource = useAlbumArtwork(albumData?.artwork_source ?? '');
	return (
		<div data-testid='albumHeaderContainer' className='albumHeaderContainer'>
			<img src={imageSource} className='albumHeaderImage' />
			<div className='albumHeaderDetails'>
				<span className='albumTitle'>{albumData.name}</span>
				<span>{albumData.year}</span>
				<span>{albumData.album_artist_name}</span>
				<span>{formatTimeDuration(albumData.duration_in_seconds)}</span>
			</div>
		</div>
	);
}

export default AlbumHeader;
