use music_database::{Album, Track};
use audio_player::{AppState, AudioPlayer};
use serde::{Deserialize, Serialize};
use tauri::{State, Builder, Manager, AppHandle, Emitter};

mod music_database;
mod audio_player;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_genres() -> Vec<String> {
    music_database::get_genres()
}

#[tauri::command]
fn get_album_artists_for_genre(genre: String) -> Vec<String> {
    music_database::get_album_artists_for_genre(genre)
}

#[tauri::command]
fn get_albums_for_album_artist(album_artist: String) -> Vec<String> {
    music_database::get_albums_for_album_artist(album_artist)
}

#[tauri::command]
fn get_album_data(album: String) -> music_database::Album {
    music_database::get_album_data(album)
}

#[tauri::command]
async fn on_track_double_clicked(app: AppHandle, state: State<'_, AppState>, track: Track) -> Result<i32, ()> {
    state.audio_player.stop();
    state.audio_player.add_track_to_queue(track);
    state.audio_player.play(app);

    Ok(1)
}

#[tauri::command]
async fn on_album_double_clicked(app: AppHandle, state: State<'_, AppState>, album: Album) -> Result<i32,()> {
    state.audio_player.stop();
    for track in album.tracks {
        state.audio_player.add_track_to_queue(track);
    }
    state.audio_player.play(app);
    Ok(1)
}

#[derive(Serialize, Deserialize, Clone)]
struct NowPlayingData {
    track_queue: Vec<Track>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            app.manage(AppState {
                audio_player: AudioPlayer::new(),
            });
            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Webview,
                ))
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_genres,
            get_album_artists_for_genre,
            get_albums_for_album_artist,
            get_album_data,
            on_track_double_clicked,
            on_album_double_clicked,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
