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
    const [artworkSource, setArtworkSource] = useState("");

    useEffect(() => {
        async function getTracksForAlbum(album: string): Promise<void> {
            const tracks: string[] = await invoke("get_tracks_for_album", { album });
            setTracks(tracks);
        }
        async function getAlbumArtwork(album: string): Promise<void> {
            const artworkSource: string = await invoke("get_artwork_for_album", {album});
            setArtworkSource(artworkSource);
        }

        getTracksForAlbum(album);
        getAlbumArtwork(album);
    },[album]);

    return (
        <div key={album} className={isSelected ? "selectedAlbum" : "albumContainer"} onClick={selectAlbum}>
            <img src={artworkSource} width="100%" height="100%" />
        </div>
    );
}

export default AlbumCard;