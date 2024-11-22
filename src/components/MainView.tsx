import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Sidebar from "./Sidebar/Sidebar";
import MainPane from "./MainPane";

interface MainViewProps {
    selectedGenre: string,
}

function MainView(props: MainViewProps) {
    const { selectedGenre } = props;

    const [albumArtists, setAlbumArtists] = useState([""]);
    const [selectedAlbumArtistIndex, setSelectedAlbumArtistIndex] = useState(0);

    useEffect(() => {
        setSelectedAlbumArtistIndex(0);
    }, [selectedGenre])

    useEffect(() => {
        async function getAlbumArtists(genre: string): Promise<void> {
            const albumArtists: string[] = await invoke("get_album_artists_for_genre", { genre });
            setAlbumArtists(albumArtists);
        }

        getAlbumArtists(selectedGenre);
    }, [selectedGenre, setAlbumArtists]);

    return (
        <div className="mainViewContainer">
            <Sidebar albumArtists={albumArtists} selectedAlbumArtistIndex={selectedAlbumArtistIndex} setSelectedAlbumArtistIndex={setSelectedAlbumArtistIndex} />

            <MainPane selectedAlbumArtist={albumArtists[selectedAlbumArtistIndex]} />
        </div>
    );
}

export default MainView;