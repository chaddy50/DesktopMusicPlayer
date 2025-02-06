import TrackData from "../../dataObjects/TrackData";

interface NowPlayingProps {
    playingTracks: TrackData[];
    playingTrackIndex: number;
}

function NowPlaying(props: NowPlayingProps) {
    const { playingTracks, playingTrackIndex } = props;

    return (
        <div>
            {playingTracks?.map((track, index) => {
                return (
                    <p
                        className={
                            index === playingTrackIndex
                                ? "selectedArtistCard"
                                : "unselectedArtistCard"
                        }
                    >
                        {track.name}
                    </p>
                );
            })}
        </div>
    );
}

export default NowPlaying;
