import { useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { formatTimeDuration } from "../../utilities/Utilities";
import TrackData from "../../dataObjects/TrackData";

interface TrackProps {
    track: TrackData;
}

function Track(props: TrackProps) {
    const { track } = props;

    const playTrack = useCallback(() => {
        invoke("on_track_double_clicked", { track });
    }, [track]);

    return (
        <div className="trackContainer" onDoubleClick={playTrack}>
            <span className="trackInfoColumnTrackNumber">
                {track.track_number}
            </span>
            <span className="trackInfoColumn">{track.name}</span>
            <span className="trackInfoColumn">{track.artist_name}</span>
            <span className="trackInfoColumnDuration">
                {formatTimeDuration(track.duration_in_seconds)}
            </span>
        </div>
    );
}

export default Track;
