import AlbumData from "../../dataObjects/AlbumData";
import "../../MusicPlayer.css";
import AlbumHeader from "./AlbumHeader";
import Track from "./Track";

interface TrackBrowserProps {
    albumData: AlbumData;
}

function TrackBrowser(props: TrackBrowserProps) {
    const { albumData } = props;

    return (
        <div id="trackBrowser" className="trackBrowserContainer">
            <AlbumHeader albumData={albumData} />
            <div className="trackListContainer">
                {albumData?.tracks.map((track) => {
                    return <Track key={track.file_path} track={track} />;
                })}
            </div>
        </div>
    );
}

export default TrackBrowser;
