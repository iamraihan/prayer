//! Alarm sound: a soft chime arpeggio synthesized at runtime (no audio asset
//! needed) and looped via rodio until answered.

use std::sync::mpsc;
use std::thread;

use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use rodio::buffer::SamplesBuffer;

enum Cmd {
    Play(f32),
    Stop,
}

/// Handle to a dedicated audio thread.
///
/// rodio's `OutputStream` is neither `Send` nor `Sync` (it wraps a cpal
/// stream), so it cannot live inside the shared engine state. Instead, the
/// stream and sink are owned by this background thread and driven by commands
/// sent through the channel.
pub struct Sound {
    tx: Option<mpsc::Sender<Cmd>>,
}

impl Sound {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let (ready_tx, ready_rx) = mpsc::channel();
        thread::spawn(move || {
            if let Ok((_stream, handle)) = OutputStream::try_default() {
                let _ = ready_tx.send(true);
                audio_loop(rx, &handle);
            } else {
                let _ = ready_tx.send(false);
            }
        });
        // Wait for device setup to finish so a missing device is known now.
        let ok = ready_rx.recv().unwrap_or(false);
        Self {
            tx: if ok { Some(tx) } else { None },
        }
    }

    /// Starts the chime loop, replacing any previous playback.
    pub fn play_loop(&mut self, volume: f32) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(Cmd::Play(volume));
        }
    }

    pub fn stop(&mut self) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(Cmd::Stop);
        }
    }
}

/// Runs on the audio thread until the last `Sender` is dropped.
fn audio_loop(rx: mpsc::Receiver<Cmd>, handle: &OutputStreamHandle) {
    let mut sink: Option<Sink> = None;
    for cmd in rx {
        match cmd {
            Cmd::Play(volume) => {
                if let Some(old) = sink.take() {
                    old.stop();
                }
                if let Ok(new) = Sink::try_new(handle) {
                    new.set_volume(volume.clamp(0.0, 1.0));
                    let buffer = SamplesBuffer::new(44_100, 1, chime());
                    new.append(buffer.repeat_infinite());
                    sink = Some(new);
                }
            }
            Cmd::Stop => {
                if let Some(old) = sink.take() {
                    old.stop();
                }
            }
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
