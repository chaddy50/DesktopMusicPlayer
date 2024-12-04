import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../../../MusicPlayer.css";
import AlbumHeader from "./AlbumHeader";

interface TrackBrowserProps {
    album: string;
}

export interface AlbumDataResponse {
    name: string;
    genre: string;
    album_artist: string;
    artwork_source: string;
    tracks: string[];
    year: number;
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
                        return <p>{track}</p>;
                    })}
                </div>
            </div>
        );
    }
}

export default TrackBrowser;
