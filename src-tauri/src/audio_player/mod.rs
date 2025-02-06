use std::{collections::VecDeque, sync::{mpsc::{self}, Mutex, RwLock}};
use audio_thread::AudioPlaybackCommand;
use tauri::AppHandle;
use crate::music_database::track::Track;

mod audio_thread;

pub struct AudioPlayer {
    audio_command_sender: mpsc::Sender<AudioPlaybackCommand>,
    music_queue: RwLock<VecDeque<Track>>,
    playing_track_index: RwLock<usize>,
    is_first_play: Mutex<bool>,
}

impl AudioPlayer {
    pub fn new(app_handle: AppHandle, sender: mpsc::Sender<AudioPlaybackCommand>, receiver: mpsc::Receiver<AudioPlaybackCommand>) -> Self {
        audio_thread::run(app_handle, receiver);

        Self {
            audio_command_sender: sender,
            music_queue: RwLock::new(VecDeque::new()),
            playing_track_index: RwLock::new(0),
            is_first_play: Mutex::new(true),
        }
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