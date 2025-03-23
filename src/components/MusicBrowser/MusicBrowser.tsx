import AlbumArtistData from '@/dataObjects/AlbumArtistData';
import AlbumData from '@/dataObjects/AlbumData';
import { invoke } from '@tauri-apps/api/core';
import { useEffect, useRef, useState } from 'react';
import { useParams } from 'react-router';
import LeftSidebar from './LeftSidebar/LeftSidebar';
import MainPane from './MainPane/MainPane';
import './MusicBrowser.css';
import RightSidebar from './RightSidebar/RightSidebar';

interface MusicBrowserParams {
	selectedGenreId: string;

	[key: string]: string | undefined;
}

function MusicBrowser() {
	const { selectedGenreId } = useParams<MusicBrowserParams>();

	const [albumArtists, setAlbumArtists] = useState<AlbumArtistData[]>([]);
	const [selectedAlbumArtistIndex, setSelectedAlbumArtistIndex] = useState(0);
	const [selectedAlbumArtistId, setSelectedAlbumArtistId] = useState(-1);
	const [selectedAlbumIndex, setSelectedAlbumIndex] = useState(-1);
	const [albums, setAlbums] = useState<AlbumData[]>([]);
	const albumListContainerRef = useRef<HTMLDivElement>(null);

	//#region Fetch data from database
	useEffect(() => {
		invoke('refresh_now_playing_data');
	}, []);

	useEffect(() => {
		async function getAlbumArtists(genreIdAsString: string): Promise<void> {
			const genreId = Number(genreIdAsString);
			const albumArtists: AlbumArtistData[] = await invoke(
				'get_album_artists_for_genre',
				{ genreId }
			);
			setAlbumArtists(albumArtists);
		}

		if (selectedGenreId) {
			getAlbumArtists(selectedGenreId);
		}
	}, [selectedGenreId, setAlbumArtists]);

	useEffect(() => {
		async function getAlbums(
			albumArtistId: number,
			genreIdAsString: string
		): Promise<void> {
			const genreId = Number(genreIdAsString);
			const albums: AlbumData[] = await invoke('get_albums_for_album_artist', {
				albumArtistId,
				genreId,
			});
			setAlbums(albums);
		}

		if (selectedAlbumArtistId >= 0 && selectedGenreId) {
			getAlbums(selectedAlbumArtistId, selectedGenreId);
		}
	}, [selectedAlbumArtistId, selectedGenreId, setAlbums]);
	//#endregion

	//#region Respond to user selections
	useEffect(() => {
		setSelectedAlbumArtistIndex(0);
		setSelectedAlbumIndex(-1);
		albumListContainerRef.current?.scrollTo(0, 0);
	}, [selectedGenreId]);

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
				albumArtistData={albumArtists[selectedAlbumArtistIndex]}
				selectedAlbumIndex={selectedAlbumIndex}
				setSelectedAlbumIndex={setSelectedAlbumIndex}
				albumListContainerRef={albumListContainerRef}
			/>

			<RightSidebar />
		</div>
	);
}

export default MusicBrowser;
