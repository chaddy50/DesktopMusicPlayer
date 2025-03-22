use crate::music_database::{album::Album, track::Track};
use audio_thread::AudioPlaybackCommand;
use std::{
    collections::VecDeque,
    sync::{
        mpsc::{self},
        Mutex, RwLock,
    },
};
use tauri::AppHandle;

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

    pub fn play_track(&self, track: Track, album: Album) {
        self.stop_playback();
        self.clear_queue();
        for track_to_add in album.tracks {
            if track_to_add.track_number >= track.track_number {
                self.add_track_to_queue(track_to_add);
            }
        }
        self.play_next_track();
    }

    pub fn resume(&self) {
        self.audio_command_sender
            .send(AudioPlaybackCommand::Resume)
            .unwrap();
    }

    pub fn pause(&self) {
        self.audio_command_sender
            .send(AudioPlaybackCommand::Pause)
            .unwrap();
    }

    pub fn skip_forward(&self) {
        self.audio_command_sender
            .send(AudioPlaybackCommand::SkipForward)
            .unwrap();
    }

    pub fn skip_backward(&self) {
        self.audio_command_sender
            .send(AudioPlaybackCommand::SkipBackward)
            .unwrap();
    }

    fn play_next_track(&self) {
        let mut playing_track_index = self
            .playing_track_index
            .write()
            .expect("playing_track_index should have been locked");

        let mut is_first_play = self
            .is_first_play
            .lock()
            .expect("is_first_play should have been locked");

        if *is_first_play {
            *is_first_play = false;
        } else {
            *playing_track_index += 1;
        }

        self.try_play_track(*playing_track_index);
    }

    fn play_previous_track(&self) {
        let mut playing_track_index = self
            .playing_track_index
            .write()
            .expect("playing_track_index should have been locked");

        if *playing_track_index > 0 {
            *playing_track_index -= 1;
        }

        self.try_play_track(*playing_track_index);
    }

    fn restart_track(&self) {
        let track_index_to_play = *self
            .playing_track_index
            .read()
            .expect("playing_track_index should have been read");
        self.try_play_track(track_index_to_play);
    }

    fn try_play_track(&self, track_index_to_play: usize) {
        let music_queue = self
            .music_queue
            .read()
            .expect("Music queue should have been read");

        if music_queue.len() > 0 && track_index_to_play < music_queue.len() {
            let next_track = music_queue[track_index_to_play].clone();
            self.audio_command_sender
                .send(AudioPlaybackCommand::PlayTrack(next_track.file_path))
                .expect("PlayTrack command should have been sent");
        } else {
            self.audio_command_sender
                .send(AudioPlaybackCommand::NothingToPlay)
                .expect("NothingToPlay command should have been sent");
        }
    }

    fn add_track_to_queue(&self, track: Track) {
        let mut music_queue = self
            .music_queue
            .write()
            .expect("music_queue should have been locked");
        music_queue.push_back(track);
    }

    fn stop_playback(&self) {
        self.audio_command_sender
            .send(AudioPlaybackCommand::Stop)
            .unwrap();
    }

    pub fn clear_queue(&self) {
        let mut music_queue = self
            .music_queue
            .write()
            .expect("music_queue should have been locked");
        music_queue.clear();

        let mut playing_track_index = self
            .playing_track_index
            .write()
            .expect("playing_track_index should have been locked");
        *playing_track_index = 0;

        let mut is_first_play = self
            .is_first_play
            .lock()
            .expect("is_first_play should have been locked");
        *is_first_play = true;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::Receiver;

    use super::*;

    pub const TEST_TRACK_1: &str = "test_files/alone-296348.mp3";
    pub const TEST_TRACK_2: &str = "test_files/gardens-stylish-chill-303261.mp3";

    pub fn make_test_audio_player() -> (AudioPlayer, Receiver<AudioPlaybackCommand>) {
        let (sender, receiver) = mpsc::channel();
        (
            AudioPlayer {
                audio_command_sender: sender,
                music_queue: RwLock::new(VecDeque::new()),
                playing_track_index: RwLock::new(0),
                is_first_play: Mutex::new(false),
            },
            receiver,
        )
    }

    pub fn make_test_track(name: &str, file_path: &str) -> Track {
        Track::new(
            name.to_string(),
            0,
            "".to_string(),
            0,
            "".to_string(),
            0,
            "".to_string(),
            file_path.to_string(),
            0,
            0,
            0,
            "".to_string(),
        )
    }

    #[test]
    fn clear_queue_clears_queue() {
        let (audio_player, _) = make_test_audio_player();

        let mut music_queue = audio_player.music_queue.write().unwrap();
        music_queue.push_back(make_test_track("Test1", TEST_TRACK_1));
        music_queue.push_back(make_test_track("Test2", TEST_TRACK_2));

        assert_eq!(music_queue.len(), 2);
        drop(music_queue);

        audio_player.clear_queue();

        let music_queue = audio_player.music_queue.read().unwrap();
        assert_eq!(music_queue.len(), 0);
    }

    #[test]
    fn clear_queue_resets_playing_track_index() {
        let (audio_player, _) = make_test_audio_player();

        *audio_player.playing_track_index.write().unwrap() = 4;
        audio_player.clear_queue();
        assert_eq!(*audio_player.playing_track_index.read().unwrap(), 0);
    }

    #[test]
    fn clear_queue_resets_is_first_play() {
        let (audio_player, _) = make_test_audio_player();

        assert!(!*audio_player.is_first_play.lock().unwrap());
        audio_player.clear_queue();
        assert!(*audio_player.is_first_play.lock().unwrap());
    }

    #[test]
    fn add_track_to_queue_adds_track_to_queue() {
        let (audio_player, _) = make_test_audio_player();

        assert_eq!(audio_player.music_queue.read().unwrap().len(), 0);
        audio_player.add_track_to_queue(make_test_track("Test1", TEST_TRACK_1));
        assert_eq!(audio_player.music_queue.read().unwrap().len(), 1);
        audio_player.add_track_to_queue(make_test_track("Test2", TEST_TRACK_2));
        assert_eq!(audio_player.music_queue.read().unwrap().len(), 2);
    }

    #[test]
    fn play_next_track_does_not_increment_playing_track_index_first_time() {
        let (audio_player, _receiver) = make_test_audio_player();
        *audio_player.is_first_play.lock().unwrap() = true;

        assert_eq!(*audio_player.playing_track_index.read().unwrap(), 0);
        audio_player.play_next_track();
        assert_eq!(*audio_player.playing_track_index.read().unwrap(), 0);
    }

    #[test]
    fn play_next_track_increments_playing_track_index() {
        let (audio_player, _receiver) = make_test_audio_player();

        assert_eq!(*audio_player.playing_track_index.read().unwrap(), 0);
        audio_player.play_next_track();
        assert_eq!(*audio_player.playing_track_index.read().unwrap(), 1);
    }

    #[test]
    fn play_previous_track_decrements_playing_track_index() {
        let (audio_player, _receiver) = make_test_audio_player();
        *audio_player.playing_track_index.write().unwrap() = 1;

        assert_eq!(*audio_player.playing_track_index.read().unwrap(), 1);
        audio_player.play_previous_track();
        assert_eq!(*audio_player.playing_track_index.read().unwrap(), 0);
    }

    #[test]
    fn play_previous_track_does_not_decrement_playing_track_index_if_zero() {
        let (audio_player, _receiver) = make_test_audio_player();
        *audio_player.playing_track_index.write().unwrap() = 0;

        assert_eq!(*audio_player.playing_track_index.read().unwrap(), 0);
        audio_player.play_previous_track();
        assert_eq!(*audio_player.playing_track_index.read().unwrap(), 0);
    }
}
