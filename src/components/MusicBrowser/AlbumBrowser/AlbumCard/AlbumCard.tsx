import { useAlbumArtwork, useSingleAndDoubleClick } from '@/common/Hooks';
import AlbumArtistData from '@/dataObjects/AlbumArtistData';
import AlbumData from '@/dataObjects/AlbumData';
import { invoke } from '@tauri-apps/api/core';
import { useCallback, useEffect, useRef } from 'react';
import '../AlbumBrowser.css';

interface AlbumCardProps {
	albumArtist: AlbumArtistData;
	album: AlbumData;
	isSelected: boolean;
	selectAlbum: () => void;
}

function AlbumCard(props: AlbumCardProps) {
	const { album, albumArtist, isSelected, selectAlbum } = props;

	const albumRef = useRef<HTMLDivElement>(null);

	useEffect(() => {
		if (isSelected && document.getElementById('trackBrowser')) {
			// We only want to do the scrolling after the trackBrowser has been rendered to avoid the scroll jumping around
			if (albumRef?.current) {
				albumRef.current.scrollIntoView({
					behavior: 'smooth',
					block: 'nearest',
				});
			}
		}
	}, [isSelected, albumRef]);

	const playAlbum = useCallback(() => {
		invoke('on_album_double_clicked', { album: album });
	}, [album]);

	const handleClicks = useSingleAndDoubleClick(selectAlbum, playAlbum);

	const imageSize = 300;
	const imageSource = useAlbumArtwork(album?.artwork_source ?? '');

	return (
		<div
			data-testid={'albumCard' + album.id}
			key={album.id}
			className='albumCardContainer'
			onClick={handleClicks}
			ref={albumRef}
		>
			<div
				data-testid={'albumImage' + album.id}
				className={
					isSelected ? 'albumArtworkContainerSelected' : 'albumArtworkContainer'
				}
			>
				<img
					src={imageSource}
					width={imageSize + 'px'}
					height={imageSize + 'px'}
				/>
			</div>
			<div
				style={{
					maxWidth: imageSize + 'px',
					height: '75px',
					display: 'flex',
					flexDirection: 'column',
				}}
			>
				<span className='albumTitle'>{album.name}</span>
				<span>{album.year}</span>
				{albumArtist?.id === 0 && <span>{album.album_artist_name}</span>}
			</div>
		</div>
	);
}

export default AlbumCard;
