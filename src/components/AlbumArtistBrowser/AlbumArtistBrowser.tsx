import { Dispatch, SetStateAction } from "react";
import AlbumArtistCard from "./AlbumArtistCard";
import "../../MusicPlayer.css";
import AlbumArtistData from "../../dataObjects/AlbumArtistData";

interface AlbumArtistBrowserProps {
    albumArtists: AlbumArtistData[];
    selectedAlbumArtistIndex: number;
    setSelectedAlbumArtistIndex: Dispatch<SetStateAction<number>>;
}

function AlbumArtistBrowser(props: AlbumArtistBrowserProps) {
    const {
        albumArtists,
        selectedAlbumArtistIndex,
        setSelectedAlbumArtistIndex,
    } = props;

    return (
        <div className="artistBrowserContainer">
            {albumArtists.map((albumArtistData, index) => {
                return (
                    <AlbumArtistCard
                        key={albumArtistData.id}
                        albumArtistData={albumArtistData}
                        isSelected={selectedAlbumArtistIndex === index}
                        selectArtist={() => setSelectedAlbumArtistIndex(index)}
                    />
                );
            })}
        </div>
    );
}

export default AlbumArtistBrowser;
