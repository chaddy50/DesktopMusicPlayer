import { useState } from "react";
import NowPlaying from "../NowPlaying/NowPlaying";
import PlayerControls from "../PlayerControls/PlayerControls";
import { listen } from "@tauri-apps/api/event";
import TrackData from "../../dataObjects/TrackData";

interface RightSidebarProps {}

interface NowPlayingData {
    playing_tracks: TrackData[];
    playing_track_index: number;
    is_paused: boolean;
    is_playing: boolean;
}

function RightSidebar(_props: RightSidebarProps) {
    const [nowPlayingData, setNowPlayingData] = useState<NowPlayingData>();

    listen<NowPlayingData>("now_playing_changed", (event) => {
        setNowPlayingData(event.payload);
    });

    if (nowPlayingData) {
        return (
            <div className="rightSidebar">
                <PlayerControls
                    isPaused={nowPlayingData.is_paused}
                    isPlaying={nowPlayingData.is_playing}
                    playing_track={
                        nowPlayingData.playing_tracks[
                            nowPlayingData.playing_track_index
                        ]
                    }
                />
                <NowPlaying
                    playingTracks={nowPlayingData.playing_tracks}
                    playingTrackIndex={nowPlayingData.playing_track_index}
                />
            </div>
        );
    }
    return <div className="rightSidebar"></div>;
}

export default RightSidebar;
