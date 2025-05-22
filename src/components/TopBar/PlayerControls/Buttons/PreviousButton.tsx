import SkipPreviousIcon from '@mui/icons-material/SkipPrevious';
import { invoke } from '@tauri-apps/api/core';

interface PreviousButtonProps {}

function PreviousButton(_props: PreviousButtonProps) {
	return (
		<div
			onClick={() => {
				invoke('on_previous_button_clicked');
			}}
		>
			<SkipPreviousIcon className='playerControlsButton' fontSize='large' />
		</div>
	);
}

export default PreviousButton;
