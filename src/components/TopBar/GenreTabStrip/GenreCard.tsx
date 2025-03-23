import GenreData from '@/dataObjects/GenreData';
import './GenreTabStrip.css';

interface GenreCardProps {
	genreData: GenreData;
	isSelected: boolean;
	selectTab: () => void;
}

function GenreCard(props: GenreCardProps) {
	const { genreData, isSelected, selectTab } = props;

	return (
		<div className='genreTab' onClick={selectTab}>
			<p className={isSelected ? 'selectedTab' : 'unselectedTab'}>
				{genreData.name}
			</p>
		</div>
	);
}

export default GenreCard;
