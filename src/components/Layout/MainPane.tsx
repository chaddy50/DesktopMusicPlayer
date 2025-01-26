import { Dispatch, RefObject, SetStateAction } from "react";
import TrackBrowser from "../TrackBrowser/TrackBrowser";
import AlbumBrowser from "../AlbumBrowser/AlbumBrowser";
import AlbumData from "../../dataObjects/AlbumData";
import AlbumArtistData from "../../dataObjects/AlbumArtistData";

interface MainPaneProps {
    albums: AlbumData[];
    albumArtistData: AlbumArtistData;
    selectedAlbumIndex: number;
    setSelectedAlbumIndex: Dispatch<SetStateAction<number>>;
    albumListContainerRef: RefObject<HTMLDivElement>;
}

function MainPane(props: MainPaneProps) {
    const {
        albums,
        albumArtistData,
        selectedAlbumIndex,
        setSelectedAlbumIndex,
        albumListContainerRef,
    } = props;

    return (
        <div className="mainPaneContainer">
            <AlbumBrowser
                albumListContainerRef={albumListContainerRef}
                albums={albums}
                albumArtistData={albumArtistData}
                selectedAlbumIndex={selectedAlbumIndex}
                setSelectedAlbumIndex={setSelectedAlbumIndex}
            />

            {selectedAlbumIndex > -1 && (
                <TrackBrowser albumData={albums[selectedAlbumIndex]} />
            )}
        </div>
    );
}

export default MainPane;
