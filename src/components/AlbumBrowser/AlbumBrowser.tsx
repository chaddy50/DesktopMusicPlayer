import { Dispatch, SetStateAction } from "react";
import AlbumCard from "./AlbumCard";

interface AlbumBrowserProps {
    albums: string[];
    selectedAlbumIndex: number;
    setSelectedAlbumIndex: Dispatch<SetStateAction<number>>;
}

function AlbumBrowser(props: AlbumBrowserProps) {
    const { albums, selectedAlbumIndex, setSelectedAlbumIndex } = props;

    return (
        <div className="albumListContainer">
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
