import { Dispatch, RefObject, SetStateAction } from "react";
import AlbumCard from "./AlbumCard";

interface AlbumBrowserProps {
    albums: string[];
    selectedAlbumIndex: number;
    setSelectedAlbumIndex: Dispatch<SetStateAction<number>>;
    albumListContainerRef: RefObject<HTMLDivElement>;
}

function AlbumBrowser(props: AlbumBrowserProps) {
    const {
        albums,
        selectedAlbumIndex,
        setSelectedAlbumIndex,
        albumListContainerRef,
    } = props;

    return (
        <div className="albumListContainer" ref={albumListContainerRef}>
            {albums.map((album, index) => {
                const indexToSelect = index === selectedAlbumIndex ? -1 : index;
                return (
                    <AlbumCard
                        album={album}
                        isSelected={index === selectedAlbumIndex}
                        selectAlbum={() => setSelectedAlbumIndex(indexToSelect)}
                    />
                );
            })}
        </div>
    );
}

export default AlbumBrowser;
