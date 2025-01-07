import { invoke } from "@tauri-apps/api/core";

interface PlayerControlsProps {
    isPlaying: boolean;
    isPaused: boolean;
}

function PlayerControls(props: PlayerControlsProps) {
    const { isPlaying } = props;

    return (
        <div>
            {isPlaying ? (
                <button
                    onClick={() => {
                        invoke("on_pause_button_clicked");
                    }}
                >
                    Pause
                </button>
            ) : (
                <button
                    onClick={() => {
                        invoke("on_play_button_clicked");
                    }}
                >
                    Play
                </button>
            )}
        </div>
    );
}

export default PlayerControls;
