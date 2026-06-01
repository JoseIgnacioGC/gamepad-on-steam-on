use std::mem::size_of;

use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Threading::{
    CreateProcessW, PROCESS_INFORMATION, STARTF_USESHOWWINDOW, STARTUPINFOW,
};
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWMINIMIZED;
use windows::core::{PCWSTR, PWSTR};

use crate::util::{to_wide, wide_to_string};

pub fn is_steam_running() -> bool {
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    let snapshot = match snapshot {
        Ok(handle) => handle,
        Err(_) => return false,
    };

    let mut entry = PROCESSENTRY32W {
        dwSize: size_of::<PROCESSENTRY32W>() as u32,
        ..Default::default()
    };

    let mut found = false;
    let mut has_entry = unsafe { Process32FirstW(snapshot, &mut entry).is_ok() };
    while has_entry {
        let exe = wide_to_string(&entry.szExeFile);
        if exe.eq_ignore_ascii_case("steam.exe") {
            found = true;
            break;
        }
        has_entry = unsafe { Process32NextW(snapshot, &mut entry).is_ok() };
    }

    unsafe {
        let _ = CloseHandle(snapshot);
    }

    found
}

pub fn launch_steam_minimized(steam_path: &str) -> bool {
    let app = to_wide(steam_path);
    let mut cmd = to_wide(&format!("\"{}\" -silent", steam_path));

    let startup = STARTUPINFOW {
        cb: size_of::<STARTUPINFOW>() as u32,
        dwFlags: STARTF_USESHOWWINDOW,
        wShowWindow: SW_SHOWMINIMIZED.0 as u16,
        ..Default::default()
    };

    let mut process = PROCESS_INFORMATION::default();

    let launched = unsafe {
        CreateProcessW(
            PCWSTR(app.as_ptr()),
            Some(PWSTR(cmd.as_mut_ptr())),
            None,
            None,
            false,
            Default::default(),
            None,
            None,
            &startup,
            &mut process,
        )
        .is_ok()
    };

    if launched {
        unsafe {
            let _ = CloseHandle(process.hProcess);
            let _ = CloseHandle(process.hThread);
        }
    }

    launched
}
