import { invoke } from "@tauri-apps/api/core";
import "./MusicPlayer.css";
import { useEffect, useState } from "react";
import TabStrip from "./components/TabStrip/TabStrip";

function MusicPlayer() {
  const [genres, setGenres] = useState([""]);
  const [selectedGenre, setSelectedGenre] = useState(0);
  const [albumArtists, setAlbumArtists] = useState([""]);

  useEffect(() => {
    async function getGenres(): Promise<void> {
      const genres: string[] = await invoke("get_genres");
      setGenres(genres);
    }

    console.log("got genres");
    getGenres();
  }, []);

  useEffect(() => {
    async function getAlbumArtists(genre: string): Promise<void> {
      const albumArtists: string[] = await invoke("get_album_artists_for_genre", { genre });
      setAlbumArtists(albumArtists);
    }

    console.log("got album artists");
    getAlbumArtists(genres[selectedGenre]);
  }, [genres, selectedGenre, setAlbumArtists]);

  console.log("albumArtists: {}", albumArtists);

  return (
    <div className="appContainer">
      <div className="topBar">
        <TabStrip tabOptions={genres} selectedTab={selectedGenre} selectTab={setSelectedGenre} />
      </div>
      <div className="mainViewContainer">
        <div className="sideBar">
          <h2>Sidebar</h2>
        </div>
        <div className="mainPane">
          <h2>Main Pane</h2>
        </div>
      </div>
      <div className="bottomBar">
        <h1>Bottom Bar</h1>
      </div>
    </div>
  );
}

export default MusicPlayer;
