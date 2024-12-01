import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

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
            <div>
                <div className="albumHeader">
                    <img
                        src={albumData.artwork_source}
                        height="100px"
                        width="100px"
                    />
                    {album}
                    {albumData.year}
                </div>
                {albumData.tracks.map((track, index) => {
                    return <p>{track}</p>;
                })}
            </div>
        );
    }
}

export default TrackBrowser;
