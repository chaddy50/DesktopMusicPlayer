import GenreData from '@/dataObjects/GenreData';
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

		getGenres();
	}, []);

	const navigate = useNavigate();

	return (
		<div className='genreBrowserContainer'>
			{genres.map((genre) => {
				return (
					<div
						className='genreBrowserCard'
						onClick={() => navigate(`musicBrowser/${genre.id}`)}
					>
						<p>{genre.name}</p>
					</div>
				);
			})}
		</div>
	);
}

export default Home;
