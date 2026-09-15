use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::path::Path;
use std::time::{Duration, Instant};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::error::Result;

pub struct AudioEngine {
    _stream: Option<OutputStream>,
    handle: Option<OutputStreamHandle>,
    sink: Option<Sink>,
    volume: f32,
    started_at: Option<Instant>,
    paused_position: Duration,
}

impl AudioEngine {
    pub fn new() -> Result<Self> {
        let (stream, handle) = OutputStream::try_default()?;
        Ok(AudioEngine {
            _stream: Some(stream),
            handle: Some(handle),
            sink: None,
            volume: 0.7,
            started_at: None,
            paused_position: Duration::ZERO,
        })
    }

    pub fn play_file(&mut self, path: &Path) -> Result<Option<Duration>> {
        self.stop();

        let file = File::open(path)?;
        let source = Decoder::new(file)?;
        let duration = source
            .total_duration()
            .or_else(|| duration_from_metadata(path));

        let sink = Sink::try_new(self.handle.as_ref().unwrap())?;
        sink.set_volume(self.volume);
        sink.append(source);

        self.sink = Some(sink);
        self.started_at = Some(Instant::now());
        self.paused_position = Duration::ZERO;
        Ok(duration)
    }

    pub fn pause(&mut self) {
        if let Some(sink) = &self.sink {
            sink.pause();
            self.paused_position = self.position();
            self.started_at = None;
        }
    }

    pub fn resume(&mut self) {
        if let Some(sink) = &self.sink {
            sink.play();
            self.started_at = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        if let Some(sink) = self.sink.take() {
            sink.stop();
        }
        self.started_at = None;
        self.paused_position = Duration::ZERO;
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
        let sink_position = self.sink.as_ref().map(Sink::get_pos).unwrap_or_default();
        if sink_position > Duration::ZERO {
            return sink_position;
        }

        if let Some(started_at) = self.started_at {
            return self.paused_position + started_at.elapsed();
        }

        self.paused_position
    }

    pub fn has_sink(&self) -> bool {
        self.sink.is_some()
    }
}

fn duration_from_metadata(path: &Path) -> Option<Duration> {
    let file = File::open(path).ok()?;
    let source = MediaSourceStream::new(Box::new(file), Default::default());
    let mut hint = Hint::new();
    if let Some(extension) = path.extension().and_then(|extension| extension.to_str()) {
        hint.with_extension(extension);
    }

    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            source,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .ok()?;

    probed
        .format
        .tracks()
        .iter()
        .filter_map(|track| {
            let frames = track.codec_params.n_frames?;
            let time_base = track.codec_params.time_base?;
            let seconds = frames as f64 * f64::from(time_base.numer) / f64::from(time_base.denom);
            Duration::try_from_secs_f64(seconds).ok()
        })
        .max()
}
