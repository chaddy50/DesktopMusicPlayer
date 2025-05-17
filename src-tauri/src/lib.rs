use audio_player::AudioPlayer;
use database::{
    music_database::{self, album::Album, album_artist::AlbumArtist, genre::Genre, track::Track},
    settings_database,
};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    AppHandle, Builder, Emitter, Manager, State,
};

pub mod audio_player;
pub mod database;
pub mod schema;

pub struct AppState {
    pub audio_player: AudioPlayer,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_genres() -> Vec<Genre> {
    music_database::get_genres()
}

#[tauri::command]
fn get_album_artists_for_genre(genre_id: i32) -> Vec<AlbumArtist> {
    music_database::get_album_artists_for_genre(&genre_id)
}

#[tauri::command]
fn get_albums_for_album_artist(album_artist_id: i32, genre_id: i32) -> Vec<Album> {
    music_database::get_albums_for_album_artist(&album_artist_id, &genre_id)
}

#[tauri::command]
fn on_track_double_clicked(state: State<'_, AppState>, track: Track, album: Album) {
    state.audio_player.play_track(track, album);
}

#[tauri::command]
fn on_album_double_clicked(state: State<'_, AppState>, album: Album) {
    state.audio_player.play_album(album);
}

#[tauri::command]
fn on_pause_button_clicked(state: State<'_, AppState>) {
    state.audio_player.pause();
}

#[tauri::command]
fn on_play_button_clicked(state: State<'_, AppState>) {
    state.audio_player.resume();
}

#[tauri::command]
fn on_next_button_clicked(state: State<'_, AppState>) {
    state.audio_player.skip_forward();
}

#[tauri::command]
fn on_previous_button_clicked(state: State<'_, AppState>) {
    state.audio_player.skip_backward();
}

#[tauri::command]
fn save_settings(directories: Vec<String>) {
    settings_database::save_settings(directories);
}

#[tauri::command]
fn load_settings(app_handle: AppHandle) {
    settings_database::load_settings(app_handle);
}

#[tauri::command]
fn rebuild_music_database() {
    music_database::rebuild();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            app.manage(AppState {
                audio_player: AudioPlayer::new(app.app_handle().clone()),
            });

            let settings = MenuItemBuilder::new("Settings...")
                .id("settings")
                .build(app)?;

            let app_submenu = SubmenuBuilder::new(app, "App").item(&settings).build()?;

            let menu = MenuBuilder::new(app).items(&[&app_submenu]).build()?;

            let _ = app.set_menu(menu);

            app.on_menu_event(move |app_handle, event| match event.id().0.as_str() {
                "settings" => {
                    app_handle.emit("open_settings", ()).unwrap();
                }
                _ => {}
            });

            load_settings(app.app_handle().clone());

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
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_genres,
            get_album_artists_for_genre,
            get_albums_for_album_artist,
            on_track_double_clicked,
            on_album_double_clicked,
            on_pause_button_clicked,
            on_play_button_clicked,
            on_next_button_clicked,
            on_previous_button_clicked,
            save_settings,
            load_settings,
            rebuild_music_database,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
