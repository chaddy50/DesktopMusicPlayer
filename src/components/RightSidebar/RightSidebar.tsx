import SettingsIcon from '@mui/icons-material/Settings';
import { useNavigate } from 'react-router';
import NowPlaying from './NowPlaying/NowPlaying';
import './RightSidebar.css';

interface RightSidebarProps {}

function RightSidebar(_props: RightSidebarProps) {
	const navigate = useNavigate();

	return (
		<div data-testid='rightSidebar' className='rightSidebar'>
			<NowPlaying />

			<div onClick={() => navigate('settings')} className='bottomTab'>
				<SettingsIcon />
			</div>
		</div>
	);
}

export default RightSidebar;
