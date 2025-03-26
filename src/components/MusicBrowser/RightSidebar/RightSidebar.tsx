import NowPlaying from './NowPlaying/NowPlaying';
import './RightSidebar.css';

interface RightSidebarProps {}

function RightSidebar(_props: RightSidebarProps) {
	return (
		<div data-testid='rightSidebar' className='rightSidebar'>
			<NowPlaying />
		</div>
	);
}

export default RightSidebar;
