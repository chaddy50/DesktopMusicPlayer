import TrackData from "../../dataObjects/TrackData";
import { formatTimeDuration } from "../../utilities/Utilities";

interface NowPlayingProps {
    playingTracks: TrackData[];
    playingTrackIndex: number;
}

function NowPlaying(props: NowPlayingProps) {
    const { playingTracks, playingTrackIndex } = props;

    return (
        <div className="trackListContainer">
            {playingTracks?.map((track, index) => {
                return (
                    <div
                        className="trackContainer"
                        style={
                            playingTrackIndex === index
                                ? {
                                      fontWeight: "bold",
                                  }
                                : {}
                        }
                    >
                        <span className="trackInfoColumnTrackNumber">
                            {index + 1}
                        </span>
                        <span className="trackInfoColumn">{track.name}</span>
                        <span className="trackInfoColumnDuration">
                            {formatTimeDuration(track.duration_in_seconds)}
                        </span>
                    </div>
                );
            })}
        </div>
    );
}

export default NowPlaying;
