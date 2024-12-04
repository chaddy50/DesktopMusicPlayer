import { invoke } from "@tauri-apps/api/core";
import { Dispatch, SetStateAction, useEffect, useState } from "react";
import AlbumCard from "./AlbumCard";
import TrackBrowser from "./TrackBrowser/TrackBrowser";

interface MainPaneProps {
    selectedAlbumArtist: string;
    selectedAlbumIndex: number;
    setSelectedAlbumIndex: Dispatch<SetStateAction<number>>;
}

function MainPane(props: MainPaneProps) {
    const { selectedAlbumArtist, selectedAlbumIndex, setSelectedAlbumIndex } =
        props;

    const [albums, setAlbums] = useState([""]);

    useEffect(() => {
        async function getAlbums(albumArtist: string): Promise<void> {
            const albums: string[] = await invoke(
                "get_albums_for_album_artist",
                { albumArtist }
            );
            setAlbums(albums);
        }
        getAlbums(selectedAlbumArtist);
    }, [selectedAlbumArtist, setAlbums]);

    return (
        <div className="mainPaneContainer">
            <div className="albumListContainer">
                {albums.map((album, index) => {
                    const indexToSelect =
                        index === selectedAlbumIndex ? -1 : index;
                    return (
                        <AlbumCard
                            album={album}
                            isSelected={index === selectedAlbumIndex}
                            selectAlbum={() =>
                                setSelectedAlbumIndex(indexToSelect)
                            }
                        />
                    );
                })}
            </div>
            {selectedAlbumIndex > -1 && (
                <TrackBrowser album={albums[selectedAlbumIndex]} />
            )}
        </div>
    );
}

export default MainPane;
