import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../../MusicPlayer.css";
import AlbumHeader from "./AlbumHeader";
import Track from "./Track";

interface TrackBrowserProps {
    album: string;
}

export interface AlbumData {
    name: string;
    genre: string;
    album_artist: string;
    artwork_source: string;
    tracks: TrackData[];
    year: number;
}

export interface TrackData {
    name: string;
    album_artist: string;
    artist: string;
    genre: string;
    artwork_source: string;
    file_path: string;
    track_number: number;
    duration_in_seconds: number;
}

function TrackBrowser(props: TrackBrowserProps) {
    const { album } = props;
    const [albumData, setAlbumData] = useState<AlbumData>();

    useEffect(() => {
        async function getAlbumData(album: string): Promise<void> {
            const albumData: AlbumData = await invoke("get_album_data", {
                album,
            });
            setAlbumData(albumData);
        }
        getAlbumData(album);
    }, [album]);

    if (albumData) {
        return (
            <div className="trackBrowserContainer">
                <AlbumHeader albumData={albumData} />
                <div className="trackListContainer">
                    {albumData.tracks.map((track) => {
                        return <Track key={track.file_path} track={track} />;
                    })}
                </div>
            </div>
        );
    }
}

export default TrackBrowser;
