import { Dispatch, RefObject, SetStateAction } from "react";
import TrackBrowser, { AlbumData } from "../TrackBrowser/TrackBrowser";
import AlbumBrowser from "../AlbumBrowser/AlbumBrowser";

interface MainPaneProps {
    albums: AlbumData[];
    selectedAlbumIndex: number;
    setSelectedAlbumIndex: Dispatch<SetStateAction<number>>;
    albumListContainerRef: RefObject<HTMLDivElement>;
}

function MainPane(props: MainPaneProps) {
    const {
        albums,
        selectedAlbumIndex,
        setSelectedAlbumIndex,
        albumListContainerRef,
    } = props;

    return (
        <div className="mainPaneContainer">
            <AlbumBrowser
                albumListContainerRef={albumListContainerRef}
                albums={albums}
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
