import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../../MusicPlayer.css";
import { AlbumData } from "../TrackBrowser/TrackBrowser";
import { useSingleAndDoubleClick } from "../../hooks/SingleAndDoubleClick";

interface AlbumCardProps {
    album: string;
    isSelected: boolean;
    selectAlbum: () => void;
}

function AlbumCard(props: AlbumCardProps) {
    const { album, isSelected, selectAlbum } = props;
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

    const playAlbum = useCallback(() => {
        invoke("on_album_double_clicked", { album: albumData });
    }, [album, albumData, selectAlbum]);

    const handleClicks = useSingleAndDoubleClick(selectAlbum, playAlbum);

    const imageSize = 300;

    if (albumData) {
        return (
            <div
                key={album}
                className="albumCardContainer"
                onClick={handleClicks}
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
        );
    }
}

export default AlbumCard;
