import "../../MusicPlayer.css";

interface ArtistCardProps {
    artist: string;
    isSelected: boolean;
    selectArtist(): void;
}

function ArtistCard(props: ArtistCardProps) {
    const { artist, isSelected, selectArtist } = props;

    return (
        <div className="artistCard" onClick={selectArtist}>
            <p
                className={
                    isSelected ? "selectedArtistCard" : "unselectedArtistCard"
                }
            >
                {artist}
            </p>
        </div>
    );
}

export default ArtistCard;
