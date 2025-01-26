import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../../MusicPlayer.css";
import AlbumHeader from "./AlbumHeader";
import Track from "./Track";

interface TrackBrowserProps {
    albumData: AlbumData;
}

export interface AlbumData {
    id: number;
    name: string;
    genre_id: number;
    genre_name: string;
    album_artist_id: number;
    album_artist_name: string;
    artwork_source: string;
    tracks: TrackData[];
    year: number;
    duration_in_seconds: number;
}

export interface TrackData {
    name: string;
    album_artist_id: number;
    album_artist_name: string;
    artist_id: number;
    artist_name: string;
    genre_id: number;
    genre_name: string;
    artwork_source: string;
    file_path: string;
    track_number: number;
    duration_in_seconds: number;
}

function TrackBrowser(props: TrackBrowserProps) {
    const { albumData } = props;

    return (
        <div id="trackBrowser" className="trackBrowserContainer">
            <AlbumHeader albumData={albumData} />
            <div className="trackListContainer">
                {albumData?.tracks.map((track) => {
                    return <Track key={track.file_path} track={track} />;
                })}
            </div>
        </div>
    );
}

export default TrackBrowser;
