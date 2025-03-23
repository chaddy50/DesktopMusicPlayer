import GenreData from '@/dataObjects/GenreData';
import { Dispatch, SetStateAction } from 'react';
import GenreTabStrip from './GenreTabStrip/GenreTabStrip';
import './TopBar.css';

interface TopBarProps {
	genres: GenreData[];
	selectedGenreIndex: number;
	setSelectedGenreIndex: Dispatch<SetStateAction<number>>;
}

function TopBar(props: TopBarProps) {
	const { genres, selectedGenreIndex, setSelectedGenreIndex } = props;

	return (
		<div className='topBar'>
			<GenreTabStrip
				genres={genres}
				selectedTab={selectedGenreIndex}
				selectTab={setSelectedGenreIndex}
			/>
		</div>
	);
}

export default TopBar;
