import "../../MusicPlayer.css";

interface AlbumArtistCardProps {
    albumArtistData: AlbumArtistData;
    isSelected: boolean;
    selectArtist(): void;
}

export interface AlbumArtistData {
    id: number;
    name: string;
}

function AlbumArtistCard(props: AlbumArtistCardProps) {
    const { albumArtistData, isSelected, selectArtist } = props;

    return (
        <div className="artistCard" onClick={selectArtist}>
            <p
                className={
                    isSelected ? "selectedArtistCard" : "unselectedArtistCard"
                }
            >
                {albumArtistData.name}
            </p>
        </div>
    );
}

export default AlbumArtistCard;
