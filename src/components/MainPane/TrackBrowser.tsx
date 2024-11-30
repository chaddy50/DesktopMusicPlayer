import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface TrackBrowserProps {
    album: string;
}

function TrackBrowser(props: TrackBrowserProps) {
const { album } = props;
const [tracks, setTracks] = useState([""]);

useEffect(() => {
async function getTracksForAlbum(album: string): Promise<void> {
    const tracks: string[] = await invoke("get_tracks_for_album", { album });
    setTracks(tracks);
}

getTracksForAlbum(album);
}, [album]);

    return (
        <div>
            {tracks.map((track, index) => {
                return <p>{track}</p>;
            })}
        </div>
    );
}

export default TrackBrowser;