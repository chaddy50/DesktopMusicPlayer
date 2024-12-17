import { useEffect, useState } from "react";
import { TrackData } from "../TrackBrowser/TrackBrowser";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface NowPlayingData {
    track_queue: TrackData[];
}

interface NowPlayingProps {}

function NowPlaying(_props: NowPlayingProps) {
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
        console.log(event.payload);
        setNowPlayingData(event.payload);
    });

    return (
        <div>
            {nowPlayingData?.track_queue.map((track) => {
                return <p>{track.name}</p>;
            })}
        </div>
    );
}

export default NowPlaying;
