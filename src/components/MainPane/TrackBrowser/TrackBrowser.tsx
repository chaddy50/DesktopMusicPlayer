import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../../../MusicPlayer.css";
import AlbumHeader from "./AlbumHeader";
import Track from "./Track";

interface TrackBrowserProps {
    album: string;
}

export interface AlbumDataResponse {
    name: string;
    genre: string;
    album_artist: string;
    artwork_source: string;
    tracks: TrackDataResponse[];
    year: number;
}

export interface TrackDataResponse {
    name: string;
    album_artist: string;
    artist: string;
    genre: string;
    artwork_source: string;
    file_path: string;
    track_number: number;
}

function TrackBrowser(props: TrackBrowserProps) {
    const { album } = props;
    const [albumData, setAlbumData] = useState<AlbumDataResponse>();

    useEffect(() => {
        async function getAlbumData(album: string): Promise<void> {
            const albumData: AlbumDataResponse = await invoke(
                "get_album_data",
                {
                    album,
                }
            );
            setAlbumData(albumData);
        }
        getAlbumData(album);
    }, [album]);

    if (albumData) {
        return (
            <div className="trackBrowserContainer">
                <AlbumHeader albumData={albumData} />
                <div className="trackListContainer">
                    {albumData.tracks.map((track, index) => {
                        return <Track track={track} />;
                    })}
                </div>
            </div>
        );
    }
}

export default TrackBrowser;
