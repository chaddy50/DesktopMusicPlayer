import { TrackData } from "../TrackBrowser/TrackBrowser";

interface NowPlayingProps {
    trackQueue: TrackData[];
}

function NowPlaying(props: NowPlayingProps) {
    const { trackQueue } = props;

    return (
        <div>
            {trackQueue?.map((track) => {
                return <p>{track.name}</p>;
            })}
        </div>
    );
}

export default NowPlaying;
