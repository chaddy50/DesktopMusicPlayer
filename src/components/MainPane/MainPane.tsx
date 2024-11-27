import { invoke } from "@tauri-apps/api/core";
import { Dispatch, SetStateAction, useEffect, useState } from "react";
import AlbumCard from "./AlbumCard";

interface MainPaneProps {
    selectedAlbumArtist: string,
    selectedAlbumIndex: number,
    setSelectedAlbumIndex: Dispatch<SetStateAction<number>>,
}

function MainPane(props: MainPaneProps) {
    const { selectedAlbumArtist, selectedAlbumIndex, setSelectedAlbumIndex } = props;

    const [albums, setAlbums] = useState([""]);

    useEffect(() => {
        async function getAlbums(albumArtist: string): Promise<void> {
            const albums: string[] = await invoke("get_albums_for_album_artist", { albumArtist });
            setAlbums(albums)
        }
        getAlbums(selectedAlbumArtist);

    }, [selectedAlbumArtist, setAlbums])

    return (
        <div className="mainPane">
            {albums.map((album, index) => {
                return <AlbumCard album={album} isSelected={index === selectedAlbumIndex} selectAlbum={() => setSelectedAlbumIndex(index)} />
            })}
        </div>
    )
}

export default MainPane;