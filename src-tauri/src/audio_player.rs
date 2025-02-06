use std::{collections::VecDeque, fs::File, io::BufReader, sync::{mpsc::{self, Receiver}, Mutex, RwLock}};
use rodio::{Decoder, OutputStream, Sink};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Emitter};
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

pub struct AudioPlayer {
    audio_command_sender: mpsc::Sender<AudioPlaybackCommand>,
    music_queue: RwLock<VecDeque<Track>>,
    playing_track_index: RwLock<usize>,
    is_first_play: Mutex<bool>,
}

impl AudioPlayer {
    pub fn new(sender: mpsc::Sender<AudioPlaybackCommand>) -> Self {
        Self {
            audio_command_sender: sender,
            music_queue: RwLock::new(VecDeque::new()),
            playing_track_index: RwLock::new(0),
            is_first_play: Mutex::new(true),
        }
    }

    pub fn run_thread(&self, app_handle: AppHandle, receiver: Receiver<AudioPlaybackCommand>) {
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

    pub fn play_next_track(&self) {
        let music_queue = self.music_queue.read().expect("Music queue should have been read");
        let mut playing_track_index = self.playing_track_index.write().expect("playing_track_index should have been locked");
        let mut is_first_play = self.is_first_play.lock().expect("is_first_play should have been locked");

        if *is_first_play {
            *is_first_play = false;
        }
        else {
            *playing_track_index += 1;
        }

        if music_queue.len() > 0 {
            let next_track = music_queue[*playing_track_index].clone();
            drop(music_queue);

            self.audio_command_sender.send(AudioPlaybackCommand::Play(next_track.file_path)).unwrap();
        } 
    }

    pub fn resume(&self) {
        self.audio_command_sender.send(AudioPlaybackCommand::Resume).unwrap();
    }

    pub fn pause(&self) {
        self.audio_command_sender.send(AudioPlaybackCommand::Pause).unwrap();
    }

    pub fn stop(&self) {
        self.stop_playback();
        self.clear_queue();
    }

    pub fn add_track_to_queue(&self, track: Track) {
        let mut music_queue = self.music_queue.write().expect("music_queue should have been locked");
        music_queue.push_back(track);
    }

    fn stop_playback(&self) {
        self.audio_command_sender.send(AudioPlaybackCommand::Stop).unwrap();
    }

    fn clear_queue(&self) {
        let mut music_queue = self.music_queue.write().expect("music_queue should have been locked");
        music_queue.clear();

        let mut playing_track_index = self.playing_track_index.write().expect("playing_track_index should have been locked");
        *playing_track_index = 0;

        let mut is_first_play = self.is_first_play.lock().expect("is_first_play should have been locked");
        *is_first_play = true;
    }
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