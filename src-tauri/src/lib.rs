use music_database::{genre::Genre, album::Album, album_artist::AlbumArtist, track::Track};
use audio_player::AudioPlayer;
use tauri::{State, Builder, Manager};
use std::sync::mpsc;

pub mod music_database;
pub mod audio_player;

pub struct AppState {
    pub audio_player: AudioPlayer
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_genres() -> Vec<Genre> {
    music_database::get_genres()
}

#[tauri::command]
fn get_album_artists_for_genre(genre_id: i64) -> Vec<AlbumArtist> {
    music_database::get_album_artists_for_genre(&genre_id)
}

#[tauri::command]
fn get_albums_for_album_artist(album_artist_id: i64, genre_id: i64) -> Vec<Album> {
    music_database::get_albums_for_album_artist(&album_artist_id, &genre_id)
}

#[tauri::command]
fn on_track_double_clicked(state: State<'_, AppState>, track: Track) -> Result<i32, ()> {
    state.audio_player.stop();
    state.audio_player.add_track_to_queue(track);
    state.audio_player.play_next_track();

    Ok(1)
}

#[tauri::command]
fn on_album_double_clicked(state: State<'_, AppState>, album: Album) -> Result<i32,()> {
    state.audio_player.stop();
    for track in album.tracks {
        state.audio_player.add_track_to_queue(track);
    }
    state.audio_player.play_next_track();
    Ok(1)
}

#[tauri::command]
fn on_pause_button_clicked(state: State<'_, AppState>) {
    state.audio_player.pause();
}

#[tauri::command]
fn on_play_button_clicked(state: State<'_, AppState>) {
    state.audio_player.resume();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            let (sender, receiver) = mpsc::channel();

            app.manage(AppState {
                audio_player: AudioPlayer::new(sender)
            });

            app.handle().state::<AppState>().audio_player.run_thread(app.handle().clone(), receiver);

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
            on_track_double_clicked,
            on_album_double_clicked,
            on_pause_button_clicked,
            on_play_button_clicked,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}