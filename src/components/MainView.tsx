import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface MainViewProps {
    selectedGenre: string,
}

function MainView(props: MainViewProps) {
    const { selectedGenre } = props;

    const [albumArtists, setAlbumArtists] = useState([""]);
    const [selectedAlbumArtistIndex, setSelectedAlbumArtistIndex] = useState(0);

    useEffect(() => {
        async function getAlbumArtists(genre: string): Promise<void> {
            console.log("got album artists");
            const albumArtists: string[] = await invoke("get_album_artists_for_genre", { genre });
            setAlbumArtists(albumArtists);
        }

        getAlbumArtists(selectedGenre);
    }, [selectedGenre, setAlbumArtists]);

    return (
        <div className="mainViewContainer">
            <div className="sideBar">
                <h2>Sidebar</h2>
            </div>
            <div className="mainPane">
                <h2>Main Pane</h2>
            </div>
        </div>
    );
}

export default MainView;