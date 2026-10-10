use crossbeam_channel::Sender;
use midir::{Ignore, MidiInput};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PttEvent {
    Pressed,
    Released,
}

pub struct PttManager {
    sender: Sender<PttEvent>,
}

impl PttManager {
    pub fn new(sender: Sender<PttEvent>) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { sender })
    }

    pub fn start_listeners(&self) {
        let tx_kb = self.sender.clone();
        let tx_midi = self.sender.clone();

        // Listener de Teclado (Polling / Hook de bajo consumo en Windows)
        thread::spawn(move || {
            listen_keyboard_ptt(tx_kb);
        });

        // Listener de Entrada MIDI (Midir CC64 / Pedal)
        thread::spawn(move || {
            listen_midi_ptt(tx_midi);
        });
    }
}

#[cfg(windows)]
fn listen_keyboard_ptt(sender: Sender<PttEvent>) {
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_MENU, VK_V};

    let mut is_down = false;

    loop {
        unsafe {
            // Detección de Alt + V (VK_MENU y VK_V)
            let alt_state = GetAsyncKeyState(VK_MENU.0 as i32);
            let v_state = GetAsyncKeyState(VK_V.0 as i32);

            let alt_pressed = (alt_state as u16 & 0x8000) != 0;
            let v_pressed = (v_state as u16 & 0x8000) != 0;

            let currently_pressed = alt_pressed && v_pressed;

            if currently_pressed && !is_down {
                is_down = true;
                let _ = sender.send(PttEvent::Pressed);
            } else if !currently_pressed && is_down {
                is_down = false;
                let _ = sender.send(PttEvent::Released);
            }
        }

        thread::sleep(std::time::Duration::from_millis(15));
    }
}

#[cfg(not(windows))]
fn listen_keyboard_ptt(_sender: Sender<PttEvent>) {}

fn listen_midi_ptt(sender: Sender<PttEvent>) {
    let mut midi_in = match MidiInput::new("reaperKev_MIDI_PTT") {
        Ok(m) => m,
        Err(_) => return,
    };
    midi_in.ignore(Ignore::None);

    let ports = midi_in.ports();
    if ports.is_empty() {
        return;
    }

    // Conectar al primer puerto MIDI disponible si existe
    if let Some(port) = ports.first() {
        let is_down = Arc::new(AtomicBool::new(false));
        let is_down_clone = Arc::clone(&is_down);
        let s = sender.clone();

        let _conn = midi_in.connect(
            port,
            "reaperkev-midi-in",
            move |_timestamp, message, _| {
                // CC 64 (Sustain Pedal) o Note-On
                if message.len() >= 3 {
                    let status = message[0] & 0xF0;
                    let control = message[1];
                    let value = message[2];

                    if status == 0xB0 && control == 64 {
                        // CC64 Sustain pedal: value >= 64 es presionado, < 64 es liberado
                        if value >= 64 && !is_down_clone.load(Ordering::SeqCst) {
                            is_down_clone.store(true, Ordering::SeqCst);
                            let _ = s.send(PttEvent::Pressed);
                        } else if value < 64 && is_down_clone.load(Ordering::SeqCst) {
                            is_down_clone.store(false, Ordering::SeqCst);
                            let _ = s.send(PttEvent::Released);
                        }
                    }
                }
            },
            (),
        );

        // Mantener viva la conexión
        loop {
            thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}
