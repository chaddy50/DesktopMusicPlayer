use std::{collections::VecDeque, fs::File, io::BufReader, sync::{mpsc::Receiver, RwLock}};
use rodio::{Decoder, OutputStream, Sink};
use serde::{Serialize, Deserialize};
use tauri::{Manager, AppHandle, Emitter};
use std::thread;

use crate::{music_database::track::Track, AppState};

#[derive(Debug)]
pub enum AudioPlaybackCommand {
    Play(String),
    Pause,
    Stop,
    Resume,
}

#[derive(Serialize, Deserialize, Clone)]
struct NowPlayingData {
    playing_tracks: Vec<Track>,
    playing_track_index: i32,
    is_playing: bool,
    is_paused: bool,
}

pub fn run(app_handle: AppHandle, receiver: Receiver<AudioPlaybackCommand>) {
    thread::spawn(move || {
        let (_output_stream, output_stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&output_stream_handle).unwrap();
        let state = app_handle.state::<AppState>();
        let mut should_wait_for_command = true;

        loop {
            if let Ok(command) = receiver.try_recv() {
                println!("Command received: {:?}", command);
                match command {
                    AudioPlaybackCommand::Play(track_file_path) => {
                        if sink.len() > 0 {
                            sink.stop();
                        }
                        sink.append(decode_track(track_file_path));
                        update_now_playing_data(&app_handle, &sink, &state.audio_player.music_queue, &state.audio_player.playing_track_index);
                    },
                    AudioPlaybackCommand::Resume => {
                        if sink.is_paused() {
                            sink.play();
                            update_now_playing_data(&app_handle, &sink, &state.audio_player.music_queue, &state.audio_player.playing_track_index);
                        }
                        else {
                            println!("Tried to resume while nothing was paused");
                        }
                    }
                    AudioPlaybackCommand::Stop => {
                        sink.stop();
                        update_now_playing_data(&app_handle, &sink, &state.audio_player.music_queue, &state.audio_player.playing_track_index);
                    }
                    AudioPlaybackCommand::Pause => {
                        sink.pause();
                        update_now_playing_data(&app_handle, &sink, &state.audio_player.music_queue, &state.audio_player.playing_track_index);
                    }
                }
                should_wait_for_command = false;
            }
            else if sink.len() == 0 && !should_wait_for_command {
                println!("playing next track");
                should_wait_for_command = true;
                state.audio_player.play_next_track();
            }

            thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}

fn decode_track(track_file_path: String) -> Decoder<BufReader<File>> {
    let track_file = BufReader::new(File::open(track_file_path).unwrap());
    Decoder::new(track_file).unwrap()
}

fn update_now_playing_data(app_handle: &AppHandle, sink: &Sink, music_queue: &RwLock<VecDeque<Track>>, playing_track_index: &RwLock<usize>) {
    let mut playing_tracks: Vec<Track> = Vec::new();
    let playing_track_index = playing_track_index.read().expect("playing_track_index should have been read");
    for track in music_queue.read().expect("music_queue should have been read").iter() {
        playing_tracks.push(track.clone());
    }
    
    let now_playing_data = NowPlayingData {
        playing_tracks,
        is_playing: sink.len() > 0 && !sink.is_paused(),
        is_paused: sink.len() > 0 && sink.is_paused(),
        playing_track_index: *playing_track_index as i32,
    };
    app_handle.emit("now_playing_changed", now_playing_data).unwrap();
}