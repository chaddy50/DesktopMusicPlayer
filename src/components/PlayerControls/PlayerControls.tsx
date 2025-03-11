import TrackData from "../../dataObjects/TrackData";
import PlayPauseButton from "./PlayPauseButton";
import PreviousButton from "./PreviousButton";
import NextButton from "./NextButton";

interface PlayerControlsProps {
    isPlaying: boolean;
    playing_track: TrackData;
}

function PlayerControls(props: PlayerControlsProps) {
    const { isPlaying, playing_track } = props;

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
            <p>{playing_track ? playing_track.name : "No track"}</p>
            <div>
                <PreviousButton />
                <PlayPauseButton isPlaying={isPlaying} />
                <NextButton />
            </div>
        </div>
    );
}

export default PlayerControls;
