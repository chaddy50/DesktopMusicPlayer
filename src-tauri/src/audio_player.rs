use std::{collections::VecDeque, fs::File, io::BufReader, sync::{Mutex, MutexGuard}};

use rodio::{Sink, OutputStream, OutputStreamHandle, Decoder};

use crate::music_database::Track;

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
            music_queue: Mutex::new(VecDeque::new())
        }
    }

    pub fn play(&self) {
        if self.sink.len() > 0 {
            self.play_or_unpause();
        }
        else {
            self.play_next_track();
        }
    }

    pub fn stop(&self) {
        self.stop_playback();
        self.clear_queue();
    }

    pub fn add_track_to_queue(&self, track: Track) {
        let mut music_queue = self.get_music_queue();
        music_queue.push_back(track);
    }

    fn play_or_unpause(&self) {
        if self.sink.is_paused() {
            self.sink.play();
        }
        else if self.sink.len() > 0 {
            self.sink.sleep_until_end();
        }
    }

    fn stop_playback(&self) {
        if self.sink.len() > 0 {
            self.sink.stop();
        }
    }

    fn clear_queue(&self) {
        let mut music_queue = self.get_music_queue();
        music_queue.clear();
    }

    fn play_next_track(&self) {
        let mut music_queue = self.get_music_queue();

        let next_track = music_queue.pop_front().expect("Queue should have a next track");
        self.sink.append(self.decode_track(&next_track));

        self.play_or_unpause();
    }

    fn decode_track(&self, track: &Track) -> Decoder<BufReader<File>> {
        let track_file = BufReader::new(File::open(&track.file_path).unwrap());
        Decoder::new(track_file).unwrap()
    }

    fn get_music_queue(&self) -> MutexGuard<'_, VecDeque<Track>> {
        self.music_queue.lock().expect("Queue should have been locked")
    }
}