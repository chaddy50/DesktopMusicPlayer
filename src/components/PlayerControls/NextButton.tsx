import { invoke } from "@tauri-apps/api/core";

interface NextButtonProps {}

function NextButton(_props: NextButtonProps) {
    return (
        <button
            onClick={() => {
                invoke("on_next_button_clicked");
            }}
        >
            Next
        </button>
    );
}

export default NextButton;
