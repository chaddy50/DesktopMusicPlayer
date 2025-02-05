use std::{collections::VecDeque, fs::File, io::BufReader, sync::{mpsc, Mutex, MutexGuard}, thread};
use rodio::{Sink, OutputStream, Decoder};

use crate::music_database::track::Track;

pub struct AppState {
    pub audio_player: AudioPlayer
}

pub struct AudioPlayer {
    audio_command_sender: mpsc::Sender<AudioPlaybackCommand>,
    pub music_queue: Mutex<VecDeque<Track>>,
}

#[derive(Debug)]
enum AudioPlaybackCommand {
    Play(String),
    Pause,
    Stop,
    Resume,
}

#[derive(Debug)]
enum NowPlayingCommand {
    NothingPlaying,
}

unsafe impl Sync for AudioPlayer {}

unsafe impl Send for AudioPlayer {}

impl AudioPlayer {
    pub fn new() -> Self {
        let (main_thread_sender, audio_thread_receiver) = mpsc::channel();
        let (audio_thread_sender, now_playing_thread_receiver) = mpsc::channel::<NowPlayingCommand>();

        thread::spawn(move || {
            let (_output_stream, output_stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&output_stream_handle).unwrap();

            loop {
                if let Ok(command) = audio_thread_receiver.try_recv() {
                    println!("Command received: {:?}", command);
                    process_command(command, &sink);
                }

                if sink.len() == 0 {
                    audio_thread_sender.send(NowPlayingCommand::NothingPlaying).unwrap();
                }

                thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        thread::spawn(move || {
            loop {
                if let Ok(command) = now_playing_thread_receiver.try_recv() {
                    println!("Command received: {:?}", command);
                }
                thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        Self {
            audio_command_sender: main_thread_sender,
            music_queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn play(&self) {
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

    pub fn play_next_track(&self) {
        let mut music_queue = self.get_music_queue();
        if music_queue.len() > 0 {
            let next_track = music_queue.pop_front().expect("Queue should have a next track");
            self.update_now_playing_data(&music_queue);
            drop(music_queue);

            self.audio_command_sender.send(AudioPlaybackCommand::Play(next_track.file_path)).unwrap();
        }
    }

    fn update_now_playing_data(&self, music_queue: &MutexGuard<VecDeque<Track>>) {
        let mut now_playing_tracks: Vec<Track> = Vec::new();
        for track in music_queue.iter() {
            now_playing_tracks.push(track.clone());
        }
        
        // let now_playing_data = NowPlayingData {
        //     track_queue: now_playing_tracks,
        //     is_paused: sink.is_paused(),
        //     is_playing: *self.get_is_playing(),
        // };
        // app.emit("now_playing_changed", now_playing_data).unwrap();
    }

    fn get_music_queue(&self) -> MutexGuard<'_, VecDeque<Track>> {
        self.music_queue.lock().expect("Queue should have been locked")
    }
}

fn decode_track(track_file_path: String) -> Decoder<BufReader<File>> {
    let track_file = BufReader::new(File::open(track_file_path).unwrap());
    Decoder::new(track_file).unwrap()
}

fn process_command(command: AudioPlaybackCommand, sink: &Sink) {
    match command {
        AudioPlaybackCommand::Play(track_file_path) => {
            if sink.len() > 0 {
                sink.stop();
            }
            sink.append(decode_track(track_file_path));
        },
        AudioPlaybackCommand::Stop => {
            sink.stop();
        }
        AudioPlaybackCommand::Pause => {
            sink.pause();
        }
        _ => {}
    }
}