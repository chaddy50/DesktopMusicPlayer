import { invoke } from "@tauri-apps/api/core";
import "./MusicPlayer.css";
import { useEffect, useState } from "react";
import MainView from "./components/MainView";
import TopBar from "./components/TopBar/TopBar";

function MusicPlayer() {
  const [genres, setGenres] = useState([""]);
  const [selectedGenreIndex, setSelectedGenreIndex] = useState(0);

  useEffect(() => {
    async function getGenres(): Promise<void> {
      const genres: string[] = await invoke("get_genres");
      setGenres(genres);
    }

    getGenres();
  }, []);

  return (
    <div className="appContainer">
      <TopBar 
        genres={genres}
        selectedGenreIndex={selectedGenreIndex} 
        setSelectedGenreIndex={setSelectedGenreIndex} 
      />

      <MainView selectedGenre={genres[selectedGenreIndex]} />
    </div>
  );
}

export default MusicPlayer;
