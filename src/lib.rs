use std::ffi::c_void;
use winapi::um::winuser::{MessageBoxA, MB_OK, MB_APPLMODAL};
use winapi::ctypes::c_char;
use winapi::shared::windef::HWND;
use winapi::shared::ntdef::NULL64;

enum InfoVersion {
    KInfoVersion = 1,
}

#[repr(C)]
pub struct PluginInfo {
    info_version: u32,
    name: *const c_char,
    version: u32,
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn SKSEPlugin_Query(_: *const c_void, info: *mut PluginInfo) -> bool {
    let mut info = unsafe { &mut *info };
    info.info_version = InfoVersion::KInfoVersion as u32;
    info.name = "launch-blocker\0".as_ptr() as *const c_char;
    info.version = 1;
    return true;
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe fn SKSEPlugin_Load(_: *const c_void) -> bool {
    MessageBoxA(
        NULL64 as HWND,
        "Press OK when you're ready\0".as_ptr() as *const i8,
        "Launch Blocker Plugin\0".as_ptr() as *const i8,
        MB_OK | MB_APPLMODAL,
    );

    return true;
}
