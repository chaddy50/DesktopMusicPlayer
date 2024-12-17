import { AlbumData } from "./TrackBrowser";
import "../../MusicPlayer.css";

interface AlbumHeaderProps {
    albumData: AlbumData;
}

function AlbumHeader(props: AlbumHeaderProps) {
    const { albumData } = props;

    return (
        <div className="albumHeaderContainer">
            <img
                src={albumData.artwork_source}
                className="albumHeaderImage"
                height="300px"
                width="300px"
            />
            <div className="albumHeaderDetails">
                <span className="albumTitle">{albumData.name}</span>
                <span>{albumData.year}</span>
                <span>{albumData.album_artist}</span>
            </div>
        </div>
    );
}

export default AlbumHeader;
