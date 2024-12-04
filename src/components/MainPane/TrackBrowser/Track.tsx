import { TrackDataResponse } from "./TrackBrowser";

interface TrackProps {
    track: TrackDataResponse;
}

function Track(props: TrackProps) {
    const { track } = props;

    return (
        <div className="trackContainer" onClick={() => {}}>
            {track.name}
        </div>
    );
}

export default Track;
