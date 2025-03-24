import { invoke } from "@tauri-apps/api/core";

interface PreviousButtonProps {}

function PreviousButton(_props: PreviousButtonProps) {
    return (
        <button
            onClick={() => {
                invoke("on_previous_button_clicked");
            }}
        >
            Previous
        </button>
    );
}

export default PreviousButton;
