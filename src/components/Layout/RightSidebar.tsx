import { useEffect, useState } from "react";
import NowPlaying from "../NowPlaying/NowPlaying";
import PlayerControls from "../PlayerControls/PlayerControls";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { TrackData } from "../TrackBrowser/TrackBrowser";

interface RightSidebarProps {}

interface NowPlayingData {
    track_queue: TrackData[];
    is_paused: boolean;
    is_playing: boolean;
}

function RightSidebar(_props: RightSidebarProps) {
    const [nowPlayingData, setNowPlayingData] = useState<NowPlayingData>();

    useEffect(() => {
        async function getNowPlayingData(): Promise<void> {
            const nowPlayingData: NowPlayingData = await invoke(
                "get_now_playing_data"
            );
            setNowPlayingData(nowPlayingData);
        }
        getNowPlayingData();
    }, [setNowPlayingData]);

    listen<NowPlayingData>("now_playing_changed", (event) => {
        setNowPlayingData(event.payload);
    });

    if (nowPlayingData) {
        return (
            <div className="rightSidebar">
                <PlayerControls
                    isPaused={nowPlayingData.is_paused}
                    isPlaying={nowPlayingData.is_playing}
                />
                <NowPlaying trackQueue={nowPlayingData.track_queue} />
            </div>
        );
    }
}

export default RightSidebar;
