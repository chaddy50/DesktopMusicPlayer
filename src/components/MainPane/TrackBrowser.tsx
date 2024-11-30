import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface TrackBrowserProps {
    album: string;
}

function TrackBrowser(props: TrackBrowserProps) {
    const { album } = props;
    const [tracks, setTracks] = useState([""]);
    const [artworkSource, setArtworkSource] = useState("");

    useEffect(() => {
        async function getTracksForAlbum(album: string): Promise<void> {
            const tracks: string[] = await invoke("get_tracks_for_album", {
                album,
            });
            setTracks(tracks);
        }
        async function getAlbumArtwork(album: string): Promise<void> {
            const artworkSource: string = await invoke(
                "get_artwork_for_album",
                {
                    album,
                }
            );
            setArtworkSource(artworkSource);
        }
        getTracksForAlbum(album);
        getAlbumArtwork(album);
    }, [album]);

    return (
        <div>
            <div>
                <img src={artworkSource} height="100px" width="100px" />
            </div>
            {tracks.map((track, index) => {
                return <p>{track}</p>;
            })}
        </div>
    );
}

export default TrackBrowser;
