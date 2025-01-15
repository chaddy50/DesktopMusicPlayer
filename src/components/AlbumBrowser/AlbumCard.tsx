import { useCallback, useEffect, useRef, useState } from "react";
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

    const albumRef = useRef<HTMLDivElement>(null);

    if (isSelected && document.getElementById("trackBrowser")) {
        // We only want to do the scrolling after the trackBrowser has been rendered to avoid the scroll jumping around
        albumRef?.current?.scrollIntoView({
            behavior: "smooth",
            block: "end",
        });
    }

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
    }, [albumData]);

    const handleClicks = useSingleAndDoubleClick(selectAlbum, playAlbum);

    const imageSize = 300;

    if (albumData) {
        return (
            <div
                key={album}
                className="albumCardContainer"
                onClick={handleClicks}
                ref={albumRef}
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
                <div
                    style={{
                        maxWidth: imageSize + "px",
                        height: "75px",
                        display: "flex",
                        flexDirection: "column",
                    }}
                >
                    <span className="albumTitle">{album}</span>
                    <span>{albumData.year}</span>
                </div>
            </div>
        );
    }
}

export default AlbumCard;
