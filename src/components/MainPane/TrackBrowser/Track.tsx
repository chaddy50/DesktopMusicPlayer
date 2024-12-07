import { useCallback } from "react";
import { TrackDataResponse } from "./TrackBrowser";
import { invoke } from "@tauri-apps/api/core";

interface TrackProps {
    track: TrackDataResponse;
}

function Track(props: TrackProps) {
    const { track } = props;

    const playTrack = useCallback(() => {
        const trackFilePath = track.file_path;
        invoke("play_track", { trackFilePath });
    }, [track]);

    return (
        <div className="trackContainer" onClick={playTrack}>
            {track.name}
        </div>
    );
}

export default Track;
