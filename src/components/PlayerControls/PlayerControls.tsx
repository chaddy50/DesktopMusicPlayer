interface PlayerControlsProps {
    isPlaying: boolean;
    isPaused: boolean;
}

function PlayerControls(props: PlayerControlsProps) {
    const { isPlaying } = props;

    return <div>{isPlaying ? <p>Pause</p> : <p>Play</p>}</div>;
}

export default PlayerControls;
