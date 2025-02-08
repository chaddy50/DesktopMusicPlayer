use rodio::{Decoder, OutputStream, Sink};
use serde::{Deserialize, Serialize};
use std::thread;
use std::{fs::File, io::BufReader, sync::mpsc::Receiver, time::Duration};
use tauri::{AppHandle, Emitter, Manager, State};

use crate::{music_database::track::Track, AppState};

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
                audio_thread.process_command(&command, &sink);
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

    fn process_command(&mut self, command: &AudioPlaybackCommand, sink: &Sink) {
        println!("Command received: {:?}", command);
        self.should_wait_for_command = false;

        match command {
            AudioPlaybackCommand::PlayTrack(track_file_path) => {
                if sink.len() > 0 {
                    sink.stop();
                }
                sink.append(self.decode_track(track_file_path));

                if sink.is_paused() {
                    sink.play();
                }
            }
            AudioPlaybackCommand::Resume => {
                if sink.is_paused() {
                    sink.play();
                } else {
                    println!("Tried to resume while nothing was paused");
                }
            }
            AudioPlaybackCommand::Stop => {
                sink.stop();
            }
            AudioPlaybackCommand::Pause => {
                sink.pause();
            }
            AudioPlaybackCommand::SkipForward => {
                sink.stop();
                self.state.audio_player.play_next_track();
            }
            AudioPlaybackCommand::SkipBackward => {
                let track_position = sink.get_pos();
                sink.stop();
                if track_position > Duration::from_secs(5) {
                    self.state.audio_player.restart_track();
                } else {
                    self.state.audio_player.play_previous_track();
                }
            }
            AudioPlaybackCommand::NothingToPlay => {
                self.should_wait_for_command = true;

                self.state.audio_player.clear_queue();
            }
        }

        self.update_now_playing_data(sink);
    }

    fn on_no_command_received(&self) {
        if !self.should_wait_for_command && self.sink.len() == 0 {
            self.state.audio_player.play_next_track();
        }
    }

    fn decode_track(&self, track_file_path: &String) -> Decoder<BufReader<File>> {
        let track_file = BufReader::new(File::open(track_file_path).unwrap());
        Decoder::new(track_file).unwrap()
    }

    fn update_now_playing_data(&self, sink: &Sink) {
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
            is_playing: sink.len() > 0 && !sink.is_paused(),
            is_paused: sink.len() > 0 && sink.is_paused(),
            playing_track_index: *playing_track_index as i32,
        };
        self.app_handle
            .emit("now_playing_changed", now_playing_data)
            .unwrap();
    }
}
