import AlbumData from "../../dataObjects/AlbumData";
import "../../MusicPlayer.css";
import AlbumHeader from "./AlbumHeader";
import Track from "./Track";

interface TrackBrowserProps {
    album: AlbumData;
}

function TrackBrowser(props: TrackBrowserProps) {
    const { album } = props;

    return (
        <div id="trackBrowser" className="trackBrowserContainer">
            <AlbumHeader albumData={album} />
            <div className="trackListContainer">
                {album?.tracks.map((track) => {
                    return <Track key={track.file_path} track={track} album={album}/>;
                })}
            </div>
        </div>
    );
}

export default TrackBrowser;
