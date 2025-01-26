import { invoke } from "@tauri-apps/api/core";
import {
    Dispatch,
    RefObject,
    SetStateAction,
    useEffect,
    useState,
} from "react";
import TrackBrowser, { AlbumData } from "../TrackBrowser/TrackBrowser";
import AlbumBrowser from "../AlbumBrowser/AlbumBrowser";

interface MainPaneProps {
    selectedAlbumArtistId: number;
    selectedAlbumIndex: number;
    setSelectedAlbumIndex: Dispatch<SetStateAction<number>>;
    selectedGenreID: number;
    albumListContainerRef: RefObject<HTMLDivElement>;
}

function MainPane(props: MainPaneProps) {
    const {
        selectedAlbumArtistId,
        selectedAlbumIndex,
        setSelectedAlbumIndex,
        selectedGenreID: selectedGenre,
        albumListContainerRef,
    } = props;

    const [albums, setAlbums] = useState<AlbumData[]>([]);

    useEffect(() => {
        async function getAlbums(
            albumArtistId: number,
            genreId: number
        ): Promise<void> {
            const albums: AlbumData[] = await invoke(
                "get_albums_for_album_artist",
                { albumArtistId, genreId }
            );
            setAlbums(albums);
        }
        getAlbums(selectedAlbumArtistId, selectedGenre);
    }, [selectedAlbumArtistId, setAlbums, selectedGenre]);

    return (
        <div className="mainPaneContainer">
            <AlbumBrowser
                albumListContainerRef={albumListContainerRef}
                albums={albums}
                selectedAlbumIndex={selectedAlbumIndex}
                setSelectedAlbumIndex={setSelectedAlbumIndex}
            />

            {selectedAlbumIndex > -1 && (
                <TrackBrowser albumData={albums[selectedAlbumIndex]} />
            )}
        </div>
    );
}

export default MainPane;
