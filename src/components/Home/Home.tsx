import GenreData from '@/dataObjects/GenreData';
import { selectedGenreStore } from '@/state/SelectedGenreStore';
import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router';
import './Home.css';

function Home() {
	const [genres, setGenres] = useState<GenreData[]>([]);

	useEffect(() => {
		async function getGenres(): Promise<void> {
			const genres: GenreData[] = await invoke('get_genres');
			setGenres(genres);
		}
		invoke('load_settings');
		getGenres();
	}, []);

	const navigate = useNavigate();

	const selectGenre = (selectedGenre: GenreData) => {
		selectedGenreStore.update(selectedGenre);
		navigate('musicBrowser');
	};

	return (
		<div className='genreBrowserContainer'>
			{genres.map((genre) => {
				return (
					<div
						key={genre.id}
						className='genreBrowserCard'
						onClick={() => selectGenre(genre)}
					>
						<p>{genre.name}</p>
					</div>
				);
			})}
		</div>
	);
}

export default Home;
