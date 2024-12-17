import NowPlaying from "../NowPlaying/NowPlaying";
import PlayerControls from "../PlayerControls/PlayerControls";

interface RightSidebarProps {}

function RightSidebar(_props: RightSidebarProps) {
    return (
        <div className="rightSidebar">
            <PlayerControls isPaused={false} isPlaying={true} />
            <NowPlaying />
        </div>
    );
}

export default RightSidebar;
