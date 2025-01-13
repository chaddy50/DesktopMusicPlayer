// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod music_database;
mod file_scanner;

fn main() {
    file_scanner::build_music_database();
    music_player_lib::run();
}
