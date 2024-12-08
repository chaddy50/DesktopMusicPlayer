import { Dispatch, SetStateAction } from "react";
import ArtistBrowser from "../ArtistBrowser/ArtistBrowser";

interface LeftSidebarProps {
    albumArtists: string[];
    selectedAlbumArtistIndex: number;
    setSelectedAlbumArtistIndex: Dispatch<SetStateAction<number>>;
}

function LeftSidebar(props: LeftSidebarProps) {
    const {
        albumArtists,
        selectedAlbumArtistIndex,
        setSelectedAlbumArtistIndex,
    } = props;

    return (
        <div className="leftSideBar">
            <ArtistBrowser
                albumArtists={albumArtists}
                selectedAlbumArtistIndex={selectedAlbumArtistIndex}
                setSelectedAlbumArtistIndex={setSelectedAlbumArtistIndex}
            />
        </div>
    );
}

export default LeftSidebar;
