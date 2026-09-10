use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::path::Path;
use std::time::Duration;

use crate::error::Result;

pub struct AudioEngine {
    _stream: Option<OutputStream>,
    handle: Option<OutputStreamHandle>,
    sink: Option<Sink>,
    volume: f32,
}

impl AudioEngine {
    pub fn new() -> Result<Self> {
        let (stream, handle) = OutputStream::try_default()?;
        Ok(AudioEngine {
            _stream: Some(stream),
            handle: Some(handle),
            sink: None,
            volume: 0.7,
        })
    }

    pub fn play_file(&mut self, path: &Path) -> Result<Option<Duration>> {
        self.stop();

        let file = File::open(path)?;
        let source = Decoder::new(file)?;
        let duration = source.total_duration();

        let sink = Sink::try_new(self.handle.as_ref().unwrap())?;
        sink.set_volume(self.volume);
        sink.append(source);

        self.sink = Some(sink);
        Ok(duration)
    }

    pub fn pause(&self) {
        if let Some(sink) = &self.sink {
            sink.pause();
        }
    }

    pub fn resume(&self) {
        if let Some(sink) = &self.sink {
            sink.play();
        }
    }

    pub fn stop(&mut self) {
        if let Some(sink) = self.sink.take() {
            sink.stop();
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        if let Some(sink) = &self.sink {
            sink.set_volume(self.volume);
        }
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn position(&self) -> Duration {
        self.sink.as_ref().map(Sink::get_pos).unwrap_or_default()
    }

    pub fn has_sink(&self) -> bool {
        self.sink.is_some()
    }
}
