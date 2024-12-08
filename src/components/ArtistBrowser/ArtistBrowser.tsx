import { Dispatch, SetStateAction } from "react";
import ArtistCard from "./ArtistCard";
import "../../MusicPlayer.css";

interface ArtistBrowserProps {
    albumArtists: string[];
    selectedAlbumArtistIndex: number;
    setSelectedAlbumArtistIndex: Dispatch<SetStateAction<number>>;
}

function ArtistBrowser(props: ArtistBrowserProps) {
    const {
        albumArtists,
        selectedAlbumArtistIndex,
        setSelectedAlbumArtistIndex,
    } = props;

    return (
        <div className="artistBrowserContainer">
            {albumArtists.map((albumArtist, index) => {
                if (albumArtist !== "") {
                    return (
                        <ArtistCard
                            artist={albumArtist}
                            isSelected={selectedAlbumArtistIndex === index}
                            selectArtist={() =>
                                setSelectedAlbumArtistIndex(index)
                            }
                        />
                    );
                }
            })}
        </div>
    );
}

export default ArtistBrowser;
