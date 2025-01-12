import { useCallback } from "react";
import { TrackData } from "./TrackBrowser";
import { invoke } from "@tauri-apps/api/core";

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
            <span className="trackInfoColumnDuration">
                {formatTimeDuration(track.duration_in_seconds)}
            </span>
        </div>
    );
}

function formatTimeDuration(timeDurationInSeconds: number): string {
    const minutes = Math.floor(timeDurationInSeconds / 60);
    let seconds = timeDurationInSeconds % 60;
    let secondsAsString = "";
    if (seconds < 10) {
        secondsAsString += "0";
    }
    secondsAsString += seconds;

    return `${minutes}:${secondsAsString}`;
}

export default Track;
