import "./MusicPlayer.css";
import BottomBar from "./components/BottomBar";
import LeftSidebar from "./components/LeftSidebar";
import MainPane from "./components/MainPane";
import TopBar from "./components/TopBar";

function MusicPlayer() {
  return (
    <div className="appContainer">
      <TopBar />
      <div className="mainView">
        <LeftSidebar />
        <MainPane />
      </div>
      <BottomBar />
    </div>
  );
}

export default MusicPlayer;
