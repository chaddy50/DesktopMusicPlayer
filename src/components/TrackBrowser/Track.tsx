import { useCallback } from "react";
import { TrackDataResponse } from "./TrackBrowser";
import { invoke } from "@tauri-apps/api/core";

interface TrackProps {
    track: TrackDataResponse;
}

function Track(props: TrackProps) {
    const { track } = props;

    const playTrack = useCallback(() => {
        invoke("on_track_double_clicked", { track });
    }, [track]);

    return (
        <div className="trackContainer" onDoubleClick={playTrack}>
            {track.name}
        </div>
    );
}

export default Track;
