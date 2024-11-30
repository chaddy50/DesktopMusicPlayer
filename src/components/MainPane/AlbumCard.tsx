import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./MainPane.css";
import TrackBrowser from "./TrackBrowser";

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
        async function getAlbumArtwork(album: string): Promise<void> {
            const artworkSource: string = await invoke("get_artwork_for_album", {album});
            setArtworkSource(artworkSource);
        }

        getAlbumArtwork(album);
    },[album]);

    const imageSize = 300;

    return (
        <div>
            <div key={album} className="albumCardContainer" onClick={selectAlbum}>
                <div className={isSelected ? "albumArtworkContainerSelected" : "albumArtworkContainer"}>
                    <img src={artworkSource} width={imageSize+"px"} height={imageSize+"px"} />
                </div>
                <p style={{maxWidth: imageSize+"px"}}>{album}</p>
            </div>
        </div>
    );
}

export default AlbumCard;