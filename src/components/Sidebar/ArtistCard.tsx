import "./Sidebar.css";

interface ArtistCardProps {
    artist: string,
    isSelected: boolean,
    selectArtist(): void
}

function ArtistCard(props: ArtistCardProps) {
    const { artist, isSelected, selectArtist } = props;

    return (
        <div style={{ display: "flex", justifyContent: "center", width: "100%", borderBottom: "1px solid black" }} onClick={selectArtist}>
            <p className={isSelected ? "selectedArtistCard" : "unselectedArtistCard"}>{artist}</p>
        </div>
    );
}

export default ArtistCard;