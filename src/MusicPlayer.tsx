import { invoke } from "@tauri-apps/api/core";
import "./MusicPlayer.css";
import { useEffect, useRef, useState } from "react";
import TopBar from "./components/Layout/TopBar";
import LeftSidebar from "./components/Layout/LeftSidebar";
import RightSidebar from "./components/Layout/RightSidebar";
import MainPane from "./components/Layout/MainPane";
import AlbumArtistData from "./dataObjects/AlbumArtistData";
import GenreData from "./dataObjects/GenreData";
import AlbumData from "./dataObjects/AlbumData";

function MusicPlayer() {
    const [genres, setGenres] = useState<GenreData[]>([]);
    const [selectedGenreIndex, setSelectedGenreIndex] = useState(0);
    const [selectedGenreId, setSelectedGenreId] = useState(-1);
    const [albumArtists, setAlbumArtists] = useState<AlbumArtistData[]>([]);
    const [selectedAlbumArtistIndex, setSelectedAlbumArtistIndex] = useState(0);
    const [selectedAlbumArtistId, setSelectedAlbumArtistId] = useState(-1);
    const [selectedAlbumIndex, setSelectedAlbumIndex] = useState(-1);
    const [albums, setAlbums] = useState<AlbumData[]>([]);
    const albumListContainerRef = useRef<HTMLDivElement>(null);

    //#region Fetch data from database
    useEffect(() => {
        async function getGenres(): Promise<void> {
            const genres: GenreData[] = await invoke("get_genres");
            setGenres(genres);
        }

        getGenres();
    }, []);

    useEffect(() => {
        async function getAlbumArtists(genreId: number): Promise<void> {
            const albumArtists: AlbumArtistData[] = await invoke(
                "get_album_artists_for_genre",
                { genreId }
            );
            setAlbumArtists(albumArtists);
        }

        if (selectedGenreId) {
            getAlbumArtists(selectedGenreId);
        }
    }, [selectedGenreId, setAlbumArtists]);

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

        if (selectedAlbumArtistId >= 0 && selectedGenreId > 0) {
            getAlbums(selectedAlbumArtistId, selectedGenreId);
        }
    }, [selectedAlbumArtistId, selectedGenreId, setAlbums]);
    //#endregion

    //#region Respond to user selections
    useEffect(() => {
        setSelectedAlbumArtistIndex(0);
        setSelectedAlbumIndex(-1);
        albumListContainerRef.current?.scrollTo(0, 0);
        setSelectedGenreId(genres[selectedGenreIndex]?.id);
    }, [genres, selectedGenreIndex]);

    useEffect(() => {
        setSelectedAlbumIndex(-1);
        setSelectedAlbumArtistId(albumArtists[selectedAlbumArtistIndex]?.id);
    }, [albumArtists, selectedAlbumArtistIndex]);
    //#endregion

    return (
        <div className="appContainer">
            <TopBar
                genres={genres}
                selectedGenreIndex={selectedGenreIndex}
                setSelectedGenreIndex={setSelectedGenreIndex}
            />

            <div className="mainViewContainer">
                <LeftSidebar
                    albumArtists={albumArtists}
                    selectedAlbumArtistIndex={selectedAlbumArtistIndex}
                    setSelectedAlbumArtistIndex={setSelectedAlbumArtistIndex}
                />

                <MainPane
                    albums={albums}
                    albumArtistData={albumArtists[selectedAlbumArtistIndex]}
                    selectedAlbumIndex={selectedAlbumIndex}
                    setSelectedAlbumIndex={setSelectedAlbumIndex}
                    albumListContainerRef={albumListContainerRef}
                />

                <RightSidebar />
            </div>
        </div>
    );
}

export default MusicPlayer;
