import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import MainPane from "./MainPane/MainPane";
import LeftSidebar from "./LeftSidebar/LeftSidebar";
import "../MusicPlayer.css";
import RightSidebar from "./RightSidebar/RightSidebar";

interface MainViewProps {
    selectedGenre: string,
}

function MainView(props: MainViewProps) {
    const { selectedGenre } = props;

    const [albumArtists, setAlbumArtists] = useState([""]);
    const [selectedAlbumArtistIndex, setSelectedAlbumArtistIndex] = useState(0);
    const [selectedAlbumIndex, setSelectedAlbumIndex] = useState(-1);

    useEffect(() => {
        setSelectedAlbumArtistIndex(0);
        setSelectedAlbumIndex(-1);
    }, [selectedGenre])

    useEffect(() => {
        setSelectedAlbumIndex(-1);
    }, [selectedAlbumArtistIndex])

    useEffect(() => {
        async function getAlbumArtists(genre: string): Promise<void> {
            const albumArtists: string[] = await invoke("get_album_artists_for_genre", { genre });
            setAlbumArtists(albumArtists);
        }

        getAlbumArtists(selectedGenre);
    }, [selectedGenre, setAlbumArtists]);

    return (
        <div className="mainViewContainer">
            <LeftSidebar albumArtists={albumArtists} selectedAlbumArtistIndex={selectedAlbumArtistIndex} setSelectedAlbumArtistIndex={setSelectedAlbumArtistIndex} />

            <MainPane selectedAlbumArtist={albumArtists[selectedAlbumArtistIndex]} selectedAlbumIndex={selectedAlbumIndex} setSelectedAlbumIndex={setSelectedAlbumIndex} />

            <RightSidebar />
        </div>
    );
}

export default MainView;