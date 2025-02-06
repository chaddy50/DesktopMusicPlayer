use std::{collections::VecDeque, fs::File, io::BufReader, sync::{mpsc::{self, Receiver}, Mutex, MutexGuard}};
use rodio::{Decoder, OutputStream, Sink};
use tauri::{AppHandle, Manager};
use std::thread;
use crate::music_database::track::Track;

#[derive(Debug)]
pub enum AudioPlaybackCommand {
    Play(String),
    Pause,
    Stop,
    Resume,
}

pub struct AppState {
    pub audio_player: AudioPlayer
}

pub struct AudioPlayer {
    audio_command_sender: mpsc::Sender<AudioPlaybackCommand>,
    music_queue: Mutex<VecDeque<Track>>,
}

impl AudioPlayer {
    pub fn new(sender: mpsc::Sender<AudioPlaybackCommand>) -> Self {
        Self {
            audio_command_sender: sender,
            music_queue: Mutex::new(VecDeque::new()),
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
                        },
                        AudioPlaybackCommand::Resume => {
                            if sink.is_paused() {
                                sink.play();
                            }
                            else {
                                println!("Tried to resume while nothing was paused");
                            }
                        }
                        AudioPlaybackCommand::Stop => {
                            sink.stop();
                        }
                        AudioPlaybackCommand::Pause => {
                            sink.pause();
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
        let mut music_queue = self.get_music_queue();
        if music_queue.len() > 0 {
            let next_track = music_queue.pop_front().expect("Queue should have a next track");
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
        let mut music_queue = self.get_music_queue();
        music_queue.push_back(track);
    }

    fn stop_playback(&self) {
        self.audio_command_sender.send(AudioPlaybackCommand::Stop).unwrap();
    }

    fn clear_queue(&self) {
        let mut music_queue = self.get_music_queue();
        music_queue.clear();
    }

    // fn update_now_playing_data(&self, music_queue: &MutexGuard<VecDeque<Track>>) {
    //     let mut now_playing_tracks: Vec<Track> = Vec::new();
    //     for track in music_queue.iter() {
    //         now_playing_tracks.push(track.clone());
    //     }
        
    //     let now_playing_data = NowPlayingData {
    //         track_queue: now_playing_tracks,
    //         is_paused: sink.is_paused(),
    //         is_playing: *self.get_is_playing(),
    //     };
    //     app.emit("now_playing_changed", now_playing_data).unwrap();
    // }

    fn get_music_queue(&self) -> MutexGuard<'_, VecDeque<Track>> {
        self.music_queue.lock().expect("Queue should have been locked")
    }
}

fn decode_track(track_file_path: String) -> Decoder<BufReader<File>> {
    let track_file = BufReader::new(File::open(track_file_path).unwrap());
    Decoder::new(track_file).unwrap()
}