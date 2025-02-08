import { invoke } from "@tauri-apps/api/core";
import TrackData from "../../dataObjects/TrackData";
import PlayPauseButton from "./PlayPauseButton";
import PreviousButton from "./PreviousButton";
import NextButton from "./NextButton";

interface PlayerControlsProps {
    isPlaying: boolean;
    isPaused: boolean;
    playing_track: TrackData;
}

function PlayerControls(props: PlayerControlsProps) {
    const { isPlaying, isPaused, playing_track } = props;

    return (
        <div
            style={{
                display: "flex",
                width: "100%",
                flexDirection: "column",
                justifyContent: "center",
                alignItems: "center",
                borderBottom: "1px solid black",
            }}
        >
            <p>{playing_track.name}</p>
            <div>
                <PreviousButton />
                <PlayPauseButton isPlaying={isPlaying} />
                <NextButton />
            </div>
        </div>
    );
}

export default PlayerControls;
