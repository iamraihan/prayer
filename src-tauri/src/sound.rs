//! Alarm sound: a soft chime arpeggio synthesized at runtime (no audio asset
//! needed) and looped via rodio until answered.

use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use rodio::buffer::SamplesBuffer;

pub struct Sound {
    /// Must stay alive for playback to continue.
    _stream: Option<OutputStream>,
    handle: Option<OutputStreamHandle>,
    sink: Option<Sink>,
}

impl Sound {
    pub fn new() -> Self {
        match OutputStream::try_default() {
            Ok((stream, handle)) => Self {
                _stream: Some(stream),
                handle: Some(handle),
                sink: None,
            },
            Err(_) => Self {
                _stream: None,
                handle: None,
                sink: None,
            },
        }
    }

    /// Starts the chime loop, replacing any previous playback.
    pub fn play_loop(&mut self, volume: f32) {
        self.stop();
        let Some(handle) = &self.handle else { return };
        if let Ok(sink) = Sink::try_new(handle) {
            sink.set_volume(volume.clamp(0.0, 1.0));
            let buffer = SamplesBuffer::new(44_100, 1, chime());
            sink.append(buffer.repeat_infinite());
            self.sink = Some(sink);
        }
    }

    pub fn stop(&mut self) {
        if let Some(sink) = self.sink.take() {
            sink.stop();
        }
    }
}

/// E5 – G#5 – B5 – E6 arpeggio with exponential decay and a breathing gap.
fn chime() -> Vec<f32> {
    let rate = 44_100.0f32;
    let note_len = (0.55 * rate) as usize;
    let notes: [(f32, usize); 4] = [
        (659.25, 0),
        (830.61, (0.38 * rate) as usize),
        (987.77, (0.76 * rate) as usize),
        (1318.51, (1.14 * rate) as usize),
    ];

    let mut samples: Vec<f32> = vec![0.0; (1.14 * rate) as usize + note_len];
    for &(freq, offset) in &notes {
        for i in 0..note_len {
            let t = i as f32 / rate;
            let envelope = (-t * 6.0).exp();
            samples[offset + i] += 0.22 * envelope * (2.0 * std::f32::consts::PI * freq * t).sin();
        }
    }
    // Short silence so the loop doesn't feel frantic.
    samples.resize(samples.len() + (0.45 * rate) as usize, 0.0);
    samples
}
