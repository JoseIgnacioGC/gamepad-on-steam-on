use std::mem::size_of;
use std::sync::OnceLock;

use windows::Win32::Devices::HumanInterfaceDevice::GUID_DEVINTERFACE_HID;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DBT_DEVICEARRIVAL, DBT_DEVTYP_DEVICEINTERFACE,
    DEV_BROADCAST_DEVICEINTERFACE_W, DEVICE_NOTIFY_WINDOW_HANDLE, DefWindowProcW, DispatchMessageW,
    GetMessageW, MSG, RegisterClassW, RegisterDeviceNotificationW, TranslateMessage,
    WM_DEVICECHANGE, WNDCLASSW, WS_OVERLAPPED,
};
use windows::core::PCWSTR;

use crate::util::to_wide;

static DEVICE_ARRIVAL_CALLBACK: OnceLock<fn()> = OnceLock::new();

pub fn set_static_device_arrival_callback(callback: fn()) {
    let _ = DEVICE_ARRIVAL_CALLBACK.set(callback);
}

pub fn create_hiden_message_window() -> Option<HWND> {
    let instance = unsafe { GetModuleHandleW(PCWSTR::null()) }.unwrap_or_default();
    let hinstance = HINSTANCE(instance.0);
    let class_name = to_wide("GamepadOnSteamOnHiddenWindow");

    let window_class = WNDCLASSW {
        lpfnWndProc: Some(notify_window_procedure),
        hInstance: hinstance,
        lpszClassName: PCWSTR(class_name.as_ptr()),
        ..Default::default()
    };

    unsafe {
        let _ = RegisterClassW(&window_class);
        CreateWindowExW(
            Default::default(),
            PCWSTR(class_name.as_ptr()),
            PCWSTR(class_name.as_ptr()),
            WS_OVERLAPPED,
            0,
            0,
            0,
            0,
            None,
            None,
            Some(hinstance),
            None,
        )
        .ok()
    }
}

pub fn register_hid_notifications(hwnd: HWND) {
    let mut filter = DEV_BROADCAST_DEVICEINTERFACE_W {
        dbcc_size: size_of::<DEV_BROADCAST_DEVICEINTERFACE_W>() as u32,
        dbcc_devicetype: DBT_DEVTYP_DEVICEINTERFACE.0,
        dbcc_reserved: 0,
        dbcc_classguid: GUID_DEVINTERFACE_HID,
        dbcc_name: [0; 1],
    };

    unsafe {
        let _ = RegisterDeviceNotificationW(
            hwnd.into(),
            &mut filter as *mut _ as *mut _,
            DEVICE_NOTIFY_WINDOW_HANDLE,
        );
    }
}

pub fn message_loop() {
    let mut msg = MSG::default();
    while unsafe { GetMessageW(&mut msg, None, 0, 0).as_bool() } {
        unsafe {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

extern "system" fn notify_window_procedure(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_DEVICECHANGE
        && wparam.0 as u32 == DBT_DEVICEARRIVAL
        && let Some(callback) = DEVICE_ARRIVAL_CALLBACK.get()
    {
        callback();
    }

    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}
