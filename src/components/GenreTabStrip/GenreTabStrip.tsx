import GenreCard, { GenreData } from "./GenreCard";

interface TabStripProps {
    genres: GenreData[];
    selectedTab: number;
    selectTab(index: number): void;
}

function GenreTabStrip(props: TabStripProps) {
    const { genres, selectedTab, selectTab } = props;

    return (
        <div style={{ display: "flex", flexDirection: "row" }}>
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
