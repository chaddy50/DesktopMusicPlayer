import { invoke } from "@tauri-apps/api/core";
import "./MusicPlayer.css";
import { useEffect, useState } from "react";
import TopBar from "./components/Layout/TopBar";
import LeftSidebar from "./components/Layout/LeftSidebar";
import RightSidebar from "./components/Layout/RightSidebar";
import MainPane from "./components/Layout/MainPane";

function MusicPlayer() {
    const [genres, setGenres] = useState([""]);
    const [selectedGenreIndex, setSelectedGenreIndex] = useState(0);
    const [albumArtists, setAlbumArtists] = useState([""]);
    const [selectedAlbumArtistIndex, setSelectedAlbumArtistIndex] = useState(0);
    const [selectedAlbumIndex, setSelectedAlbumIndex] = useState(-1);

    useEffect(() => {
        async function getGenres(): Promise<void> {
            const genres: string[] = await invoke("get_genres");
            setGenres(genres);
        }

        getGenres();
    }, []);

    useEffect(() => {
        setSelectedAlbumArtistIndex(0);
        setSelectedAlbumIndex(-1);
    }, [selectedGenreIndex]);

    useEffect(() => {
        setSelectedAlbumIndex(-1);
    }, [selectedAlbumArtistIndex]);

    useEffect(() => {
        async function getAlbumArtists(genre: string): Promise<void> {
            const albumArtists: string[] = await invoke(
                "get_album_artists_for_genre",
                { genre }
            );
            setAlbumArtists(albumArtists);
        }

        getAlbumArtists(genres[selectedGenreIndex]);
    }, [genres, selectedGenreIndex, setAlbumArtists]);

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
                    selectedAlbumArtist={albumArtists[selectedAlbumArtistIndex]}
                    selectedAlbumIndex={selectedAlbumIndex}
                    setSelectedAlbumIndex={setSelectedAlbumIndex}
                />

                <RightSidebar />
            </div>
        </div>
    );
}

export default MusicPlayer;
