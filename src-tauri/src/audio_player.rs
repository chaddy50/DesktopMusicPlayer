use std::{collections::VecDeque, fs::File, io::BufReader, sync::{Mutex, MutexGuard}};
use rodio::{Sink, OutputStream, OutputStreamHandle, Decoder};
use tauri::{AppHandle, Emitter};

use crate::{music_database::Track, NowPlayingData};

pub struct AppState {
    pub audio_player: AudioPlayer
}

pub struct AudioPlayer {
    pub sink: Sink,
    #[allow(dead_code)] 
    // The output stream needs to be kept around in order for the music to continue playing, but it's not actually used anywhere
    output_stream: OutputStream,
    #[allow(dead_code)]
    output_stream_handle: OutputStreamHandle,
    pub music_queue: Mutex<VecDeque<Track>>,
    is_playing: Mutex<bool>,
}

unsafe impl Sync for AudioPlayer {}

unsafe impl Send for AudioPlayer {}

impl AudioPlayer {
    pub fn new() -> Self {
        let (output_stream, output_stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&output_stream_handle).unwrap();
        Self {
            sink,
            output_stream,
            output_stream_handle,
            music_queue: Mutex::new(VecDeque::new()),
            is_playing: Mutex::new(false),
        }
    }

    pub fn play(&self, app: AppHandle) {
        if self.sink.len() > 0 {
            self.play_or_unpause(&app);
        }
        else {
            self.play_next_track(&app);
        }
    }

    pub fn pause(&self, app: AppHandle) {
        self.sink.pause();
        *self.get_is_playing() = false;

        self.update_now_playing_data(&app, &self.get_music_queue());
    }

    pub fn stop(&self) {
        self.stop_playback();
        self.clear_queue();
    }

    pub fn add_track_to_queue(&self, track: Track) {
        let mut music_queue = self.get_music_queue();
        music_queue.push_back(track);
    }

    fn play_or_unpause(&self, app: &AppHandle) {
        if self.sink.is_paused() {
            self.sink.play();
            *self.get_is_playing() = true;

            self.update_now_playing_data(app, &self.get_music_queue());
        }
        else if self.sink.len() > 0 {
            *self.get_is_playing() = true;
            self.update_now_playing_data(app, &self.get_music_queue());

            self.sink.sleep_until_end();
            self.play_next_track(app);
        }
    }

    fn stop_playback(&self) {
        self.sink.clear();
        self.sink.stop();

        *self.get_is_playing() = false;
    }

    fn clear_queue(&self) {
        let mut music_queue = self.get_music_queue();
        music_queue.clear();
    }

    fn play_next_track(&self, app: &AppHandle) {
        let mut music_queue = self.get_music_queue();
        if music_queue.len() > 0 {
            let next_track = music_queue.pop_front().expect("Queue should have a next track");
            self.sink.append(self.decode_track(&next_track));
            self.update_now_playing_data(app, &music_queue);
            drop(music_queue);

            self.play_or_unpause(app);
        }
    }

    fn update_now_playing_data(&self, app: &AppHandle, music_queue: &MutexGuard<VecDeque<Track>>) {
        let mut now_playing_tracks: Vec<Track> = Vec::new();
        for track in music_queue.iter() {
            now_playing_tracks.push(track.clone());
        }
        
        let now_playing_data = NowPlayingData {
            track_queue: now_playing_tracks,
            is_paused: self.sink.is_paused(),
            is_playing: *self.get_is_playing(),
        };
        app.emit("now_playing_changed", now_playing_data).unwrap();
    }

    fn decode_track(&self, track: &Track) -> Decoder<BufReader<File>> {
        let track_file = BufReader::new(File::open(&track.file_path).unwrap());
        Decoder::new(track_file).unwrap()
    }

    fn get_music_queue(&self) -> MutexGuard<'_, VecDeque<Track>> {
        self.music_queue.lock().expect("Queue should have been locked")
    }

    fn get_is_playing(&self) -> MutexGuard<'_, bool> {
        self.is_playing.lock().expect("is_playing should have been locked")
    }
}