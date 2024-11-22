import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./MainPane.css";

interface AlbumCardProps {
    album: string;
    isSelected: boolean;
    selectAlbum: () => void;
}

function AlbumCard(props: AlbumCardProps) {
    const { album, isSelected, selectAlbum} = props;
    const [tracks, setTracks] = useState([""]);

    useEffect(() => {
        async function getTracksForAlbum(album: string): Promise<void> {
            const tracks: string[] = await invoke("get_tracks_for_album", { album });
            setTracks(tracks);
        }
        getTracksForAlbum(album);
    },[]);

    console.log(tracks);

    return (
        <div key={album} style={{ width: "200px", height: "200px", display: "flex", flexDirection: "column", justifyContent: "center", alignItems: "center", border: "1px solid black", margin: "20px" }} onClick={selectAlbum}>
            <p className={isSelected ? "selectedAlbum" : "unselectedAlbum"}>{album}</p>
            {isSelected &&
                tracks.map((track) => {
                    return <p>{track}</p>
                })
            }
        </div>
    );
}

export default AlbumCard;