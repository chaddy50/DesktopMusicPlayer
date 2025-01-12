import { invoke } from "@tauri-apps/api/core";
import { Dispatch, SetStateAction, useEffect, useState } from "react";
import TrackBrowser from "../TrackBrowser/TrackBrowser";
import AlbumBrowser from "../AlbumBrowser/AlbumBrowser";

interface MainPaneProps {
    selectedAlbumArtist: string;
    selectedAlbumIndex: number;
    setSelectedAlbumIndex: Dispatch<SetStateAction<number>>;
    selectedGenre: string;
}

function MainPane(props: MainPaneProps) {
    const {
        selectedAlbumArtist,
        selectedAlbumIndex,
        setSelectedAlbumIndex,
        selectedGenre,
    } = props;

    const [albums, setAlbums] = useState([""]);

    useEffect(() => {
        async function getAlbums(
            albumArtist: string,
            genre: string
        ): Promise<void> {
            const albums: string[] = await invoke(
                "get_albums_for_album_artist",
                { albumArtist, genre }
            );
            setAlbums(albums);
        }
        getAlbums(selectedAlbumArtist, selectedGenre);
    }, [selectedAlbumArtist, setAlbums, selectedGenre]);

    return (
        <div className="mainPaneContainer">
            <AlbumBrowser
                albums={albums}
                selectedAlbumIndex={selectedAlbumIndex}
                setSelectedAlbumIndex={setSelectedAlbumIndex}
            />

            {selectedAlbumIndex > -1 && (
                <TrackBrowser album={albums[selectedAlbumIndex]} />
            )}
        </div>
    );
}

export default MainPane;
