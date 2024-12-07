use rodio::{Sink, OutputStream, OutputStreamHandle};

pub struct AppState {
    pub audio_player: Audio_Player
}

pub struct Audio_Player {
    pub sink: Sink,
    output_stream: OutputStream,
    output_stream_handle: OutputStreamHandle,
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
        }
    }
}