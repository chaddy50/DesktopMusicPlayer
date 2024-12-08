import { AlbumDataResponse } from "./TrackBrowser";

interface AlbumHeaderProps {
    albumData: AlbumDataResponse;
}

function AlbumHeader(props: AlbumHeaderProps) {
    const { albumData } = props;

    return (
        <div className="albumHeader">
            <img
                src={albumData.artwork_source}
                className="albumHeaderImage"
                height="100px"
                width="100px"
            />
            <div className="albumHeaderDetails">
                <label>{albumData.name}</label>
                <label>{albumData.year}</label>
                <label>{albumData.album_artist}</label>
            </div>
        </div>
    );
}

export default AlbumHeader;
