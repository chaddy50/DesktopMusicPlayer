import AlbumArtistData from '@/dataObjects/AlbumArtistData';
import AlbumData from '@/dataObjects/AlbumData';
import { selectedGenreStore } from '@/state/SelectedGenreStore';
import { invoke } from '@tauri-apps/api/core';
import { observer } from 'mobx-react';
import { useEffect, useRef, useState } from 'react';
import LeftSidebar from './LeftSidebar/LeftSidebar';
import MainPane from './MainPane/MainPane';
import './MusicBrowser.css';
import RightSidebar from './RightSidebar/RightSidebar';

const MusicBrowser = observer(() => {
	const selectedGenre = selectedGenreStore.genre;
	console.log('genre: ' + selectedGenre?.name);

	const [albumArtists, setAlbumArtists] = useState<AlbumArtistData[]>([]);
	const [selectedAlbumArtistIndex, setSelectedAlbumArtistIndex] = useState(0);
	const [selectedAlbumArtistId, setSelectedAlbumArtistId] = useState(-1);
	const [selectedAlbumIndex, setSelectedAlbumIndex] = useState(-1);
	const [albums, setAlbums] = useState<AlbumData[]>([]);
	const albumListContainerRef = useRef<HTMLDivElement>(null);

	//#region Fetch data from database
	useEffect(() => {
		async function getAlbumArtists(genreId: number): Promise<void> {
			const albumArtists: AlbumArtistData[] = await invoke(
				'get_album_artists_for_genre',
				{ genreId }
			);
			setAlbumArtists(albumArtists);
		}

		if (selectedGenre?.id) {
			getAlbumArtists(selectedGenre.id);
		}
	}, [selectedGenre, setAlbumArtists]);

	useEffect(() => {
		async function getAlbums(
			albumArtistId: number,
			genreId: number
		): Promise<void> {
			const albums: AlbumData[] = await invoke('get_albums_for_album_artist', {
				albumArtistId,
				genreId,
			});
			setAlbums(albums);
		}

		if (selectedAlbumArtistId >= 0 && selectedGenre?.id) {
			getAlbums(selectedAlbumArtistId, selectedGenre.id);
		}
	}, [selectedAlbumArtistId, selectedGenre, setAlbums]);
	//#endregion

	//#region Respond to user selections
	useEffect(() => {
		setSelectedAlbumArtistIndex(0);
		setSelectedAlbumIndex(-1);
		albumListContainerRef.current?.scrollTo(0, 0);
	}, [selectedGenre]);

	useEffect(() => {
		setSelectedAlbumIndex(-1);
		setSelectedAlbumArtistId(albumArtists[selectedAlbumArtistIndex]?.id);
	}, [albumArtists, selectedAlbumArtistIndex]);
	//#endregion

	return (
		<div className='musicBrowserContainer'>
			<LeftSidebar
				albumArtists={albumArtists}
				selectedAlbumArtistIndex={selectedAlbumArtistIndex}
				setSelectedAlbumArtistIndex={setSelectedAlbumArtistIndex}
			/>

			<MainPane
				albums={albums}
				albumArtist={albumArtists[selectedAlbumArtistIndex]}
				selectedAlbumIndex={selectedAlbumIndex}
				setSelectedAlbumIndex={setSelectedAlbumIndex}
				albumListContainerRef={albumListContainerRef}
			/>

			<RightSidebar />
		</div>
	);
});

export default MusicBrowser;
