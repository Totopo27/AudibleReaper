use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct AudioCapture {
    recording: Arc<AtomicBool>,
    buffer: Arc<Mutex<Vec<f32>>>,
}

impl AudioCapture {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let recording = Arc::new(AtomicBool::new(false));
        let buffer = Arc::new(Mutex::new(Vec::new()));

        let rec_clone = Arc::clone(&recording);
        let buf_clone = Arc::clone(&buffer);

        // El thread de captura posee el Stream localmente para satisfacer los bounds de CPAL
        thread::spawn(move || {
            let host = cpal::default_host();
            let Some(device) = host.default_input_device() else {
                return;
            };

            let Ok(config) = device.default_input_config() else {
                return;
            };

            let stream: Option<Stream> = match config.sample_format() {
                SampleFormat::F32 => device
                    .build_input_stream(
                        &config.into(),
                        move |data: &[f32], _: &cpal::InputCallbackInfo| {
                            if rec_clone.load(Ordering::Relaxed) {
                                if let Ok(mut lock) = buf_clone.lock() {
                                    lock.extend_from_slice(data);
                                }
                            }
                        },
                        |err| eprintln!("Error en captura de audio WASAPI: {}", err),
                        None,
                    )
                    .ok(),
                _ => None,
            };

            if let Some(ref s) = stream {
                let _ = s.play();
            }

            loop {
                thread::sleep(std::time::Duration::from_millis(500));
            }
        });

        Ok(Self { recording, buffer })
    }

    pub fn start(&self) {
        if let Ok(mut lock) = self.buffer.lock() {
            lock.clear();
        }
        self.recording.store(true, Ordering::SeqCst);
    }

    pub fn stop(&self) -> Vec<f32> {
        self.recording.store(false, Ordering::SeqCst);
        if let Ok(lock) = self.buffer.lock() {
            lock.clone()
        } else {
            Vec::new()
        }
    }
}
