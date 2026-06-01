#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod device;
mod events;
mod steam;
mod util;
mod win;

use std::sync::atomic::AtomicBool;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

const STEAM_PATH: &str = r"C:\Program Files (x86)\Steam\Steam.exe";
const COOLDOWN_SECS: u64 = 5;

static WORKER_RUNNING: AtomicBool = AtomicBool::new(false);
static LAST_TRIGGER: OnceLock<Mutex<Instant>> = OnceLock::new();

fn main() {
    events::setup_ctrl_c();

    let hwnd = match win::create_hiden_message_window() {
        Some(hwnd) => hwnd,
        None => {
            crate::debug_log!("No se pudo crear la ventana de mensajes.");
            return;
        }
    };

    crate::debug_log!("Inicializando detector de dispositivos.");
    win::set_static_device_arrival_callback(device::on_device_arrival);
    win::register_hid_notifications(hwnd);

    crate::debug_log!("Escuchando eventos de dispositivos.");
    win::message_loop();
}
