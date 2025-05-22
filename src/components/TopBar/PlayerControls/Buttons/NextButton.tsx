import SkipNextIcon from '@mui/icons-material/SkipNext';
import { invoke } from '@tauri-apps/api/core';

interface NextButtonProps {}

function NextButton(_props: NextButtonProps) {
	return (
		<div
			onClick={() => {
				invoke('on_next_button_clicked');
			}}
		>
			<SkipNextIcon className='playerControlsButton' fontSize='large' />
		</div>
	);
}

export default NextButton;
