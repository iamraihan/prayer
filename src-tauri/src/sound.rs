//! Alarm sound. The default is a soft synthesized chime; the user can pick a
//! custom tone (mp3/wav/ogg/flac) which is copied into the app config dir by
//! `commands::set_alarm_tone`.

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use rodio::buffer::SamplesBuffer;

/// Playback cap for custom tones — matches the alarm auto-stop window, so
/// decoding never buffers more than the app can actually play.
pub const MAX_TONE_SECONDS: u64 = 120;

enum Cmd {
    /// (Re)load the tone: the custom file when given and decodable, the
    /// default chime otherwise.
    Load(Option<PathBuf>),
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

    /// Starts the current tone in a loop, replacing any previous playback.
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

    /// Switches to a custom tone file (`None` = built-in chime). A file that
    /// fails to decode falls back to the chime.
    pub fn load(&mut self, custom: Option<PathBuf>) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(Cmd::Load(custom));
        }
    }
}

/// Runs on the audio thread until the last `Sender` is dropped.
fn audio_loop(rx: mpsc::Receiver<Cmd>, handle: &OutputStreamHandle) {
    let mut sink: Option<Sink> = None;
    let mut tone: Option<Tone> = None;
    for cmd in rx {
        match cmd {
            Cmd::Load(custom) => {
                tone = custom.as_deref().and_then(load_tone);
            }
            Cmd::Play(volume) => {
                if let Some(old) = sink.take() {
                    old.stop();
                }
                if let Ok(new) = Sink::try_new(handle) {
                    new.set_volume(volume.clamp(0.0, 1.0));
                    let current = tone.clone().unwrap_or_else(chime_tone);
                    // rodio's SamplesBuffer takes (channels, sample_rate, data).
                    let buffer = SamplesBuffer::new(
                        current.channels,
                        current.sample_rate,
                        current.samples,
                    );
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

/// Fully decoded, loopable audio.
#[derive(Clone)]
struct Tone {
    sample_rate: u32,
    channels: u16,
    samples: Vec<i16>,
}

/// Decodes an audio file into loopable samples, capped at
/// [`MAX_TONE_SECONDS`]. Returns `None` when the file can't be opened or
/// decoded (bad format, unsupported codec, …).
fn load_tone(path: &Path) -> Option<Tone> {
    let file = File::open(path).ok()?;
    let decoder = Decoder::new(BufReader::new(file)).ok()?;
    let sample_rate = decoder.sample_rate();
    let channels = decoder.channels();
    if sample_rate == 0 || channels == 0 || channels > 8 {
        return None;
    }
    let cap = (sample_rate as usize)
        .saturating_mul(MAX_TONE_SECONDS as usize)
        .saturating_mul(channels as usize);
    let mut samples: Vec<i16> = Vec::new();
    for sample in decoder.take(cap) {
        samples.push(sample);
    }
    if samples.is_empty() {
        return None;
    }
    Some(Tone {
        sample_rate,
        channels: channels as u16,
        samples,
    })
}

/// True when the file exists and decodes — used to validate uploads before
/// anything is persisted.
pub fn is_decodable(path: &Path) -> bool {
    load_tone(path).is_some()
}

fn chime_tone() -> Tone {
    // The synthesized chime is float; convert to the i16 sample format the
    // file decoder uses so both share one Tone type.
    Tone {
        sample_rate: 44_100,
        channels: 1,
        samples: chime()
            .into_iter()
            .map(|s: f32| (s * i16::MAX as f32) as i16)
            .collect(),
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
