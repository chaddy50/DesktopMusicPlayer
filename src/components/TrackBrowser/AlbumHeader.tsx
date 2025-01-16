import { AlbumData } from "./TrackBrowser";
import "../../MusicPlayer.css";
import { formatTimeDuration } from "../../utilities/Utilities";

interface AlbumHeaderProps {
    albumData: AlbumData;
}

function AlbumHeader(props: AlbumHeaderProps) {
    const { albumData } = props;

    return (
        <div className="albumHeaderContainer">
            <img src={albumData.artwork_source} className="albumHeaderImage" />
            <div className="albumHeaderDetails">
                <span className="albumTitle">{albumData.name}</span>
                <span>{albumData.year}</span>
                <span>{albumData.album_artist}</span>
                <span>{formatTimeDuration(albumData.duration_in_seconds)}</span>
            </div>
        </div>
    );
}

export default AlbumHeader;
