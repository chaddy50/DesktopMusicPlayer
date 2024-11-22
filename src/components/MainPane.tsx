import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

interface MainPaneProps {
    selectedAlbumArtist: string,
}

function MainPane(props: MainPaneProps) {
    const { selectedAlbumArtist } = props;

    const [albums, setAlbums] = useState([""]);

    useEffect(() => {
        async function getAlbums(albumArtist: string): Promise<void> {
            const albums: string[] = await invoke("get_albums_for_album_artist", { albumArtist });
            setAlbums(albums)
        }
        getAlbums(selectedAlbumArtist);

    }, [selectedAlbumArtist, setAlbums])

    console.log("selectedAlbumArtist: {}", selectedAlbumArtist)
    console.log(albums);
    return (
        <div className="mainPane">
            {albums.map((album) => {
                return (
                    <div key={album} style={{ width: "200px", height: "200px", display: "flex", justifyContent: "center", alignItems: "center", border: "1px solid black", margin: "20px" }}>
                        {album}
                    </div>
                );
            })}
        </div>
    )
}

export default MainPane;