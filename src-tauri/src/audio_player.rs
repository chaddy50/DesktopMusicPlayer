use std::{collections::VecDeque, fs::File, io::BufReader, ops::Deref, sync::{Mutex, MutexGuard}};

use rodio::{Sink, OutputStream, OutputStreamHandle, Decoder};

use crate::music_database::track;

pub struct AppState {
    pub audio_player: Audio_Player
}

pub struct Audio_Player {
    pub sink: Sink,
    output_stream: OutputStream,
    output_stream_handle: OutputStreamHandle,
    pub music_queue: Mutex<VecDeque<track>>,
}

unsafe impl Sync for Audio_Player {}

unsafe impl Send for Audio_Player {}

impl Audio_Player {
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

    pub fn start_playback(&self) {
        if self.sink.len() > 0 {
            self.sink.sleep_until_end();
        }
    }

    fn stop_playback(&self) {
        if self.sink.len() > 0 {
            self.sink.stop();
        }
    }

    pub fn clear_queue(&self) {
        self.stop_playback();

        let mut music_queue = self.get_music_queue();
        music_queue.clear();
        drop(music_queue);
    }

    pub fn add_track_to_queue(&self, track: track) {
        self.sink.append(self.decode_track(&track));

        let mut music_queue = self.get_music_queue();
        music_queue.push_back(track);
    }

    fn get_music_queue(&self) -> MutexGuard<'_, VecDeque<track>> {
        self.music_queue.lock().expect("Queue locked")
    }

    fn decode_track(&self, track: &track) -> Decoder<BufReader<File>> {
        let track_file = BufReader::new(File::open(&track.file_path).unwrap());
        Decoder::new(track_file).unwrap()
    }
}