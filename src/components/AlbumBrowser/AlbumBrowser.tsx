import { Dispatch, RefObject, SetStateAction } from "react";
import AlbumCard from "./AlbumCard";
import AlbumData from "../../dataObjects/AlbumData";

interface AlbumBrowserProps {
    albums: AlbumData[];
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
            {albums.map((albumData, index) => {
                const indexToSelect = index === selectedAlbumIndex ? -1 : index;
                return (
                    <AlbumCard
                        key={albumData.id}
                        albumData={albumData}
                        isSelected={index === selectedAlbumIndex}
                        selectAlbum={() => setSelectedAlbumIndex(indexToSelect)}
                    />
                );
            })}
        </div>
    );
}

export default AlbumBrowser;
