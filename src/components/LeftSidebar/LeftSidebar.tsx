import { Dispatch, SetStateAction } from "react";
import ArtistCard from "./ArtistCard";

interface LeftSidebarProps {
    albumArtists: string[],
    selectedAlbumArtistIndex: number,
    setSelectedAlbumArtistIndex: Dispatch<SetStateAction<number>>,
}

function LeftSidebar(props: LeftSidebarProps) {
    const { albumArtists, selectedAlbumArtistIndex, setSelectedAlbumArtistIndex } = props;

    return (
        <div className="sideBar">
            {albumArtists.map((albumArtist, index) => {
                if (albumArtist !== "") {
                    return <ArtistCard 
                        artist={albumArtist} 
                        isSelected={selectedAlbumArtistIndex === index} 
                        selectArtist={() => setSelectedAlbumArtistIndex(index)} 
                    />
                }
            })}
        </div>
    )
}

export default LeftSidebar;