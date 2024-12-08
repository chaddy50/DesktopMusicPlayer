use std::{borrow::Borrow, collections::VecDeque, sync::Mutex};

use music_database::track;
use tauri::{State, Builder, Manager};

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
fn get_album_data(album: String) -> music_database::album {
    music_database::get_album_data(album)
}

#[tauri::command]
async fn on_track_clicked(state: State<'_, audio_player::AppState>, track: track) -> Result<i32, ()> {
    state.audio_player.clear_queue();
    state.audio_player.add_track_to_queue(track);
    state.audio_player.start_playback();
    Ok(1)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            app.manage(audio_player::AppState {
                audio_player: audio_player::Audio_Player::new(),
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
            on_track_clicked,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
