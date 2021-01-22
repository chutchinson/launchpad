use winapi::um::winuser::{EnumWindows, EnumChildWindows, MoveWindow, SetWindowPos, GetWindowThreadProcessId, GetWindow, IsWindowVisible, GW_OWNER, HWND_TOPMOST};
use winapi::um::wingdi::{DISPLAY_DEVICEA};
use winapi::shared::windef::{HWND};
use winapi::shared::minwindef::{BOOL, LPARAM, TRUE, FALSE};
use winapi::ctypes::{c_void};
use std::time::{Instant, Duration};

unsafe extern "system" fn enumerate_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let closure: &mut &mut FnMut(HWND) -> bool = std::mem::transmute(lparam as *mut c_void);
    if closure(hwnd) { TRUE } else { FALSE }
}

pub fn enumerate_windows<F>(mut callback: F)
    where F: FnMut(HWND) -> bool
{
    let mut trait_obj: &mut dyn FnMut(HWND) -> bool = &mut callback;
    let closure_pointer_pointer: *mut c_void = unsafe { std::mem::transmute(&mut trait_obj) };

    let lparam = closure_pointer_pointer as LPARAM;
    unsafe { EnumWindows(Some(enumerate_callback), lparam) };
}

fn is_main_window(hwnd: HWND) -> bool {
    unsafe {
        return GetWindow(hwnd, GW_OWNER) == 0 as HWND && IsWindowVisible(hwnd) == 0;
    }
}

pub fn move_window(hwnd: HWND, x: i32, y: i32, width: u32, height: u32) -> bool {
    unsafe {
        let insert_after = HWND_TOPMOST; // top most
        let flags = 0;
        let result = SetWindowPos(hwnd, insertAfter,
                     x, y, width as i32, height as i32, flags);
        return result != 0;
    }
}

pub fn find_main_window(pid: u32) -> Option<HWND> {
    let mut hwnd: Option<HWND> = None;
    let start = Instant::now();
    let max_duration = Duration::from_secs(5);
    unsafe {
        while hwnd.is_none() && start.elapsed() < max_duration {
            enumerate_windows(|handle: HWND| -> bool {
                let mut window_pid = 0;
                GetWindowThreadProcessId(handle, &mut window_pid);
                if window_pid == pid && is_main_window(handle) {
                    hwnd = Some(handle);
                    return false;
                }
                return true;
            });
        }

        return hwnd;
    }
}