use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream};
use crossbeam_channel::{unbounded, Sender};
use std::f32::consts::PI;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EarconType {
    PttStart,
    PttStop,
    Success,
    Error,
}

pub struct EarconPlayer {
    sender: Sender<EarconType>,
}

impl EarconPlayer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (tx, rx) = unbounded();

        // El thread de audio posee y ejecuta el Stream de CPAL localmente (thread-bound)
        thread::spawn(move || {
            let host = cpal::default_host();
            let Some(device) = host.default_output_device() else {
                return;
            };

            let Ok(config) = device.default_output_config() else {
                return;
            };

            let sample_rate = config.sample_rate().0 as f32;
            let channels = config.channels() as usize;

            let current_samples = Arc::new(std::sync::Mutex::new(Vec::<f32>::new()));
            let sample_idx = Arc::new(std::sync::atomic::AtomicUsize::new(0));

            let samples_cb = Arc::clone(&current_samples);
            let idx_cb = Arc::clone(&sample_idx);

            let stream: Option<Stream> = match config.sample_format() {
                SampleFormat::F32 => device
                    .build_output_stream(
                        &config.into(),
                        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                            let lock = samples_cb.lock().unwrap();
                            let mut idx = idx_cb.load(Ordering::Relaxed);

                            for sample in data.iter_mut() {
                                if idx < lock.len() {
                                    *sample = lock[idx];
                                    idx += 1;
                                } else {
                                    *sample = 0.0;
                                }
                            }

                            idx_cb.store(idx, Ordering::Relaxed);
                        },
                        |err| eprintln!("Error en stream de earcons: {}", err),
                        None,
                    )
                    .ok(),
                _ => None,
            };

            if let Some(ref s) = stream {
                let _ = s.play();
            }

            while let Ok(earcon) = rx.recv() {
                let generated = generate_earcon_waveform(earcon, sample_rate, channels);
                {
                    let mut lock = current_samples.lock().unwrap();
                    *lock = generated;
                    sample_idx.store(0, Ordering::SeqCst);
                }
            }
        });

        Ok(Self { sender: tx })
    }

    pub fn play(&self, earcon: EarconType) {
        let _ = self.sender.send(earcon);
    }
}

fn generate_earcon_waveform(earcon: EarconType, sample_rate: f32, channels: usize) -> Vec<f32> {
    let (freq1, freq2, duration_ms) = match earcon {
        EarconType::PttStart => (440.0, 880.0, 100.0), // Sweep ascendente suave
        EarconType::PttStop => (880.0, 440.0, 80.0),   // Sweep descendente
        EarconType::Success => (587.33, 880.0, 150.0), // Acorde mayor agradable (D5 -> A5)
        EarconType::Error => (220.0, 180.0, 180.0),    // Tono grave bajo
    };

    let total_frames = ((duration_ms / 1000.0) * sample_rate) as usize;
    let mut buffer = Vec::with_capacity(total_frames * channels);

    for frame in 0..total_frames {
        let t = frame as f32 / sample_rate;
        let progress = frame as f32 / total_frames as f32;
        let current_freq = freq1 + (freq2 - freq1) * progress;

        // Envelope fade-in y fade-out para evitar clics acústicos
        let envelope = if progress < 0.1 {
            progress / 0.1
        } else if progress > 0.8 {
            (1.0 - progress) / 0.2
        } else {
            1.0
        };

        let sample_val = (2.0 * PI * current_freq * t).sin() * 0.25 * envelope;

        for _ in 0..channels {
            buffer.push(sample_val);
        }
    }

    buffer
}
