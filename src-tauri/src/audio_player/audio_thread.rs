use rodio::{Decoder, OutputStream, Sink};
use serde::{Deserialize, Serialize};
use std::thread;
use std::{fs::File, io::BufReader, sync::mpsc::Receiver, time::Duration};
use tauri::{AppHandle, Emitter, Manager, State};

use crate::{music_database::track::Track, AppState};

use super::AudioPlayer;

#[derive(Debug)]
pub enum AudioPlaybackCommand {
    PlayTrack(String),
    Pause,
    Stop,
    Resume,
    SkipForward,
    SkipBackward,
    NothingToPlay,
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
        let mut audio_thread = AudioThread::new(&app_handle, &state, &sink);

        loop {
            if let Ok(command) = receiver.try_recv() {
                audio_thread.process_command(&command);
            } else {
                audio_thread.on_no_command_received();
            }

            thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}

struct AudioThread<'a> {
    should_wait_for_command: bool,
    sink: &'a Sink,
    state: &'a State<'a, AppState>,
    app_handle: &'a AppHandle,
}

impl<'a> AudioThread<'a> {
    fn new(app_handle: &'a AppHandle, state: &'a State<'a, AppState>, sink: &'a Sink) -> Self {
        AudioThread {
            should_wait_for_command: true,
            sink,
            state,
            app_handle,
        }
    }

    fn process_command(&mut self, command: &AudioPlaybackCommand) {
        println!("Command received: {:?}", command);
        self.should_wait_for_command = false;

        match command {
            AudioPlaybackCommand::PlayTrack(track_file_path) => {
                on_play_track_received(self.sink, track_file_path)
            }
            AudioPlaybackCommand::Resume => {
                on_resume_received(self.sink);
            }
            AudioPlaybackCommand::Stop => {
                on_stop_recieved(self.sink);
            }
            AudioPlaybackCommand::Pause => {
                on_pause_received(self.sink);
            }
            AudioPlaybackCommand::SkipForward => {
                on_skip_forward_received(self.sink, &self.state.audio_player);
            }
            AudioPlaybackCommand::SkipBackward => {
                on_skip_backward_received(self.sink, &self.state.audio_player);
            }
            AudioPlaybackCommand::NothingToPlay => {
                self.should_wait_for_command = true;

                self.state.audio_player.clear_queue();
            }
        }

        self.update_now_playing_data();
    }

    fn on_no_command_received(&self) {
        if !self.should_wait_for_command && self.sink.len() == 0 {
            self.state.audio_player.play_next_track();
        }
    }

    fn update_now_playing_data(&self) {
        let playing_track_index = self
            .state
            .audio_player
            .playing_track_index
            .read()
            .expect("playing_track_index should have been read");
        let music_queue = self
            .state
            .audio_player
            .music_queue
            .read()
            .expect("music_queue should have been read");

        let mut playing_tracks: Vec<Track> = Vec::new();
        for track in music_queue.iter() {
            playing_tracks.push(track.clone());
        }

        let now_playing_data = NowPlayingData {
            playing_tracks,
            is_playing: self.sink.len() > 0 && !self.sink.is_paused(),
            is_paused: self.sink.len() > 0 && self.sink.is_paused(),
            playing_track_index: *playing_track_index as i32,
        };
        self.app_handle
            .emit("now_playing_changed", now_playing_data)
            .unwrap();
    }
}

fn on_play_track_received(sink: &Sink, track_file_path: &String) {
    if sink.len() > 0 {
        sink.stop();
        sink.clear();
    }
    sink.append(decode_track(track_file_path));

    if sink.is_paused() {
        sink.play();
    }
}

fn on_stop_recieved(sink: &Sink) {
    sink.stop();
    sink.clear();
}

fn on_pause_received(sink: &Sink) {
    sink.pause();
}

fn on_resume_received(sink: &Sink) {
    if sink.is_paused() {
        sink.play();
    } else {
        panic!("Tried to resume while nothing was paused");
    }
}

fn on_skip_forward_received(sink: &Sink, audio_player: &AudioPlayer) {
    sink.stop();
    audio_player.play_next_track();
}

fn on_skip_backward_received(sink: &Sink, audio_player: &AudioPlayer) {
    let track_position = sink.get_pos();
    sink.stop();
    if track_position > Duration::from_secs(5) {
        audio_player.restart_track();
    } else {
        audio_player.play_previous_track();
    }
}

fn decode_track(track_file_path: &String) -> Decoder<BufReader<File>> {
    let track_file = BufReader::new(File::open(track_file_path).unwrap());
    Decoder::new(track_file).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::audio_player::tests::{
        make_test_audio_player, make_test_track, TEST_TRACK_1, TEST_TRACK_2,
    };

    use super::*;

    #[test]
    fn stop_clears_queue() {
        let (_output_stream, output_stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&output_stream_handle).unwrap();

        sink.append(decode_track(&TEST_TRACK_1.to_string()));

        assert_eq!(sink.len(), 1);

        on_stop_recieved(&sink);

        assert_eq!(sink.len(), 0);
        assert!(sink.is_paused());
    }

    #[test]
    fn play_track_resets_queue() {
        let (_output_stream, output_stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&output_stream_handle).unwrap();

        sink.append(decode_track(&TEST_TRACK_1.to_string()));

        assert_eq!(sink.len(), 1);

        on_play_track_received(&sink, &TEST_TRACK_2.to_string());

        assert_eq!(sink.len(), 1);
    }

    #[test]
    fn play_track_unpauses() {
        let (_output_stream, output_stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&output_stream_handle).unwrap();

        sink.append(decode_track(&TEST_TRACK_1.to_string()));
        sink.pause();

        assert!(sink.is_paused());

        on_play_track_received(&sink, &TEST_TRACK_1.to_string());

        assert!(!sink.is_paused());
    }

    #[test]
    fn pause_pauses() {
        let (_output_stream, output_stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&output_stream_handle).unwrap();

        sink.append(decode_track(&TEST_TRACK_1.to_string()));

        assert!(!sink.is_paused());

        on_pause_received(&sink);

        assert!(sink.is_paused());
    }

    #[test]
    fn resume_unpauses() {
        let (_output_stream, output_stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&output_stream_handle).unwrap();

        sink.append(decode_track(&TEST_TRACK_1.to_string()));
        sink.pause();

        assert!(sink.is_paused());

        on_resume_received(&sink);

        assert!(!sink.is_paused());
    }

    #[test]
    #[should_panic]
    fn resume_panics_if_not_paused() {
        let (_output_stream, output_stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&output_stream_handle).unwrap();

        sink.append(decode_track(&TEST_TRACK_1.to_string()));

        assert!(!sink.is_paused());

        on_resume_received(&sink);
    }

    #[test]
    fn skip_forward_plays_next_track() {
        let (_output_stream, output_stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&output_stream_handle).unwrap();

        let (audio_player, _receiver) = make_test_audio_player();

        let mut music_queue = audio_player.music_queue.write().unwrap();
        music_queue.push_back(make_test_track("Test1", TEST_TRACK_1));
        music_queue.push_back(make_test_track("Test2", TEST_TRACK_2));
        drop(music_queue);

        sink.append(decode_track(&TEST_TRACK_1.to_string()));

        assert_eq!(*audio_player.playing_track_index.read().unwrap(), 0);

        on_skip_forward_received(&sink, &audio_player);

        assert_eq!(*audio_player.playing_track_index.read().unwrap(), 1);
    }

    #[test]
    fn skip_backward_plays_previous_track() {
        let (_output_stream, output_stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&output_stream_handle).unwrap();

        let (audio_player, _receiver) = make_test_audio_player();

        sink.append(decode_track(&TEST_TRACK_1.to_string()));
        *audio_player.playing_track_index.write().unwrap() = 1;

        on_skip_backward_received(&sink, &audio_player);
        assert_eq!(*audio_player.playing_track_index.read().unwrap(), 0);
    }

    #[test]
    fn skip_backward_restarts_track() {
        let (_output_stream, output_stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&output_stream_handle).unwrap();

        let (audio_player, _receiver) = make_test_audio_player();

        sink.set_volume(0.0);
        sink.append(decode_track(&TEST_TRACK_1.to_string()));
        *audio_player.playing_track_index.write().unwrap() = 1;

        thread::sleep(Duration::from_secs(6));
        on_skip_backward_received(&sink, &audio_player);
        assert_eq!(*audio_player.playing_track_index.read().unwrap(), 1);
    }
}
