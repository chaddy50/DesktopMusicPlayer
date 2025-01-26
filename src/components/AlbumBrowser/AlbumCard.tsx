import { useCallback, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../../MusicPlayer.css";
import { useSingleAndDoubleClick } from "../../hooks/SingleAndDoubleClick";
import { useAlbumArtwork } from "../../utilities/Utilities";
import AlbumData from "../../dataObjects/AlbumData";

interface AlbumCardProps {
    albumData: AlbumData;
    isSelected: boolean;
    selectAlbum: () => void;
}

function AlbumCard(props: AlbumCardProps) {
    const { albumData, isSelected, selectAlbum } = props;

    const albumRef = useRef<HTMLDivElement>(null);

    if (isSelected && document.getElementById("trackBrowser")) {
        // We only want to do the scrolling after the trackBrowser has been rendered to avoid the scroll jumping around
        albumRef?.current?.scrollIntoView({
            behavior: "smooth",
            block: "end",
        });
    }

    const playAlbum = useCallback(() => {
        invoke("on_album_double_clicked", { album: albumData });
    }, [albumData]);

    const handleClicks = useSingleAndDoubleClick(selectAlbum, playAlbum);

    const imageSize = 300;
    const imageSource = useAlbumArtwork(albumData?.artwork_source ?? "");

    return (
        <div
            key={albumData.id}
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
                    src={imageSource}
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
                <span className="albumTitle">{albumData.name}</span>
                <span>{albumData.year}</span>
            </div>
        </div>
    );
}

export default AlbumCard;
