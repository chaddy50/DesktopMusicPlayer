use std::{collections::VecDeque, sync::{mpsc::{self}, Mutex, RwLock}};
use audio_thread::AudioPlaybackCommand;
use tauri::AppHandle;
use crate::music_database::{album::Album, track::Track};

mod audio_thread;

pub struct AudioPlayer {
    audio_command_sender: mpsc::Sender<AudioPlaybackCommand>,
    music_queue: RwLock<VecDeque<Track>>,
    playing_track_index: RwLock<usize>,
    is_first_play: Mutex<bool>,
}

impl AudioPlayer {
    pub fn new(app_handle: AppHandle) -> Self {
        let (sender, receiver) = mpsc::channel();
        audio_thread::run(app_handle, receiver);

        Self {
            audio_command_sender: sender,
            music_queue: RwLock::new(VecDeque::new()),
            playing_track_index: RwLock::new(0),
            is_first_play: Mutex::new(true),
        }
    }

    pub fn play_album(&self, album: Album) {
        self.stop_playback();
        self.clear_queue();
        for track in album.tracks {
            self.add_track_to_queue(track);
        }
        self.play_next_track();
    }

    pub fn play_track(&self, track: Track) {
        self.stop_playback();
        self.clear_queue();
        self.add_track_to_queue(track);
        self.play_next_track();
    }

    pub fn resume(&self) {
        self.audio_command_sender.send(AudioPlaybackCommand::Resume).unwrap();
    }

    pub fn pause(&self) {
        self.audio_command_sender.send(AudioPlaybackCommand::Pause).unwrap();
    }

    pub fn skip_forward(&self) {
        self.audio_command_sender.send(AudioPlaybackCommand::SkipForward).unwrap();
    }

    pub fn skip_backward(&self) {
        self.audio_command_sender.send(AudioPlaybackCommand::SkipBackward).unwrap();
    }

    fn play_next_track(&self) {
        let mut playing_track_index = self.playing_track_index.write().expect("playing_track_index should have been locked");
        let mut is_first_play = self.is_first_play.lock().expect("is_first_play should have been locked");

        if *is_first_play {
            *is_first_play = false;
        }
        else {
            *playing_track_index += 1;
        }

        self.try_play_track(*playing_track_index);
    }

    fn play_previous_track(&self) {
        let mut playing_track_index = self.playing_track_index.write().expect("playing_track_index should have been locked");
        
        if *playing_track_index > 0 {
            *playing_track_index -= 1;
        }

        self.try_play_track(*playing_track_index);
    }

    fn restart_track(&self) {
        let track_index_to_play = *self.playing_track_index.read().expect("playing_track_index should have been read");
        self.try_play_track(track_index_to_play);
    }

    fn try_play_track(&self, track_index_to_play: usize) {
        let music_queue = self.music_queue.read().expect("Music queue should have been read");

        if music_queue.len() > 0 && track_index_to_play < music_queue.len() {
            let next_track = music_queue[track_index_to_play].clone();
            self.audio_command_sender.send(AudioPlaybackCommand::PlayTrack(next_track.file_path)).unwrap();
        } else {
            self.audio_command_sender.send(AudioPlaybackCommand::NothingToPlay).unwrap();
        }
    }

    fn add_track_to_queue(&self, track: Track) {
        let mut music_queue = self.music_queue.write().expect("music_queue should have been locked");
        music_queue.push_back(track);
    }

    fn stop_playback(&self) {
        self.audio_command_sender.send(AudioPlaybackCommand::Stop).unwrap();
    }

    pub fn clear_queue(&self) {
        let mut music_queue = self.music_queue.write().expect("music_queue should have been locked");
        music_queue.clear();

        let mut playing_track_index = self.playing_track_index.write().expect("playing_track_index should have been locked");
        *playing_track_index = 0;

        let mut is_first_play = self.is_first_play.lock().expect("is_first_play should have been locked");
        *is_first_play = true;
    }
}