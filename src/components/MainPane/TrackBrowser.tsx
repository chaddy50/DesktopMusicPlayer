import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./MainPane.css";

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
                <div className="albumHeader">
                    <img
                        src={albumData.artwork_source}
                        height="100px"
                        width="100px"
                    />
                    <p>{album}</p>
                    <p>{albumData.year}</p>
                </div>
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
