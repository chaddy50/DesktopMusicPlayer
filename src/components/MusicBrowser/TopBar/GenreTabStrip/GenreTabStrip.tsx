import GenreData from '@/dataObjects/GenreData';
import GenreCard from './GenreCard';
import './GenreTabStrip.css';

interface TabStripProps {
	genres: GenreData[];
	selectedTab: number;
	selectTab(index: number): void;
}

function GenreTabStrip(props: TabStripProps) {
	const { genres, selectedTab, selectTab } = props;

	return (
		<div className='genreTabStrip'>
			{genres.map((genreData, index) => {
				return (
					<GenreCard
						key={genreData.id}
						genreData={genreData}
						isSelected={index === selectedTab}
						selectTab={() => selectTab(index)}
					/>
				);
			})}
		</div>
	);
}

export default GenreTabStrip;
