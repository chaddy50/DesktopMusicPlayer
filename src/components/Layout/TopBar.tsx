import { Dispatch, SetStateAction } from "react";
import TabStrip from "../TabStrip/TabStrip";

interface TopBarProps {
    genres: string[];
    selectedGenreIndex: number;
    setSelectedGenreIndex: Dispatch<SetStateAction<number>>;
}

function TopBar(props: TopBarProps) {
    const { genres, selectedGenreIndex, setSelectedGenreIndex } = props;

    return (
        <div className="topBar">
            <TabStrip
                tabOptions={genres}
                selectedTab={selectedGenreIndex}
                selectTab={setSelectedGenreIndex}
            />
        </div>
    );
}

export default TopBar;
