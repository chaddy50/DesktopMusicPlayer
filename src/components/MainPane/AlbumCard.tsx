import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../../MusicPlayer.css";
import { AlbumDataResponse } from "./TrackBrowser/TrackBrowser";

interface AlbumCardProps {
    album: string;
    isSelected: boolean;
    selectAlbum: () => void;
}

function AlbumCard(props: AlbumCardProps) {
    const { album, isSelected, selectAlbum } = props;
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

    const imageSize = 300;

    if (albumData) {
        return (
            <div>
                <div
                    key={album}
                    className="albumCardContainer"
                    onClick={selectAlbum}
                >
                    <div
                        className={
                            isSelected
                                ? "albumArtworkContainerSelected"
                                : "albumArtworkContainer"
                        }
                    >
                        <img
                            src={albumData.artwork_source}
                            width={imageSize + "px"}
                            height={imageSize + "px"}
                        />
                    </div>
                    <p style={{ maxWidth: imageSize + "px" }}>{album}</p>
                </div>
            </div>
        );
    }
}

export default AlbumCard;
