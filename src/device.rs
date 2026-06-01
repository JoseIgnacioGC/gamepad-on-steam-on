use crate::steam::{is_steam_running, launch_steam_minimized};
use crate::{COOLDOWN_SECS, LAST_TRIGGER, STEAM_PATH, WORKER_RUNNING};

use std::sync::Mutex;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};
use windows::Win32::UI::Input::XboxController::{XINPUT_STATE, XInputGetState};

pub fn on_device_arrival() {
    if WORKER_RUNNING
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return;
    }

    std::thread::spawn(|| {
        crate::debug_log!("Evento de dispositivo recibido.");
        handle_device_arrival();
        WORKER_RUNNING.store(false, Ordering::SeqCst);
    });
}

fn has_xbox_gamepad() -> bool {
    for index in 0..4u32 {
        let mut state = XINPUT_STATE::default();
        let result = unsafe { XInputGetState(index, &mut state) };
        if result == 0 {
            crate::debug_log!("XInput mando conectado en el indice {index}.");
            return true;
        }
    }

    crate::debug_log!("No hay mandos XInput conectados.");
    false
}

fn handle_device_arrival() {
    if !cooldown_ready() {
        crate::debug_log!("Cooldown activo; se ignora el evento.");
        return;
    }

    if !has_xbox_gamepad() {
        crate::debug_log!("No hay mandos XInput detectados.");
        return;
    }

    if is_steam_running() {
        crate::debug_log!("Steam ya esta en ejecucion.");
        return;
    }

    if launch_steam_minimized(STEAM_PATH) {
        crate::debug_log!("Steam iniciado en modo minimizado.");
    } else {
        crate::debug_log!("No se pudo iniciar Steam.");
    }
}

fn cooldown_ready() -> bool {
    let now = Instant::now();
    let lock = LAST_TRIGGER.get_or_init(|| Mutex::new(now - Duration::from_secs(60)));
    let mut last = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if now.duration_since(*last) < Duration::from_secs(COOLDOWN_SECS) {
        return false;
    }
    *last = now;
    true
}
