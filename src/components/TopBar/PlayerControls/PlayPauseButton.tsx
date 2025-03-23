import { invoke } from "@tauri-apps/api/core";

interface PlayPauseButtonProps {
    isPlaying: boolean;
}

function PlayPauseButton(props: PlayPauseButtonProps) {
    const { isPlaying } = props;
    return (
        <>
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
        </>
    );
}

export default PlayPauseButton;
