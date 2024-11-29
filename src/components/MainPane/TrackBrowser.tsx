interface TrackBrowserProps {
    tracks: string[];
}

function TrackBrowser(props: TrackBrowserProps) {
const { tracks } = props;

    return (
        <div>
            {tracks.map((track, index) => {
                return <p>{track}</p>;
            })}
        </div>
    );
}

export default TrackBrowser;