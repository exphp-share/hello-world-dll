use winapi::shared::minwindef::{DWORD, BOOL, ULONG};
use winapi::ctypes::{c_char, c_void};

extern "system" fn MessageBoxThread(param: *mut c_void) -> DWORD {
    unsafe {
        winapi::um::winuser::MessageBoxA(
            std::ptr::null_mut(), // hWnd
            b"Hello world!\0" as *const u8 as *const c_char, // lpText
            b"Hello World!\0" as *const u8 as *const c_char, // lpCaption
            0, // uType
        ) as DWORD
    }
}

#[no_mangle]
extern "C" fn HelloWorld() {
    unsafe {
        winapi::um::processthreadsapi::CreateThread(
            std::ptr::null_mut(), // lpThreadAttributes
            0, // dwStackSize
            Some(MessageBoxThread), // lpStartAddress
            std::ptr::null_mut(), // lpParameter
            0, // dwCreationFlags
            std::ptr::null_mut(), // lpThreadId
        );
    }
}

#[no_mangle]
pub extern "stdcall" fn DllMain(
    _hinst_dll: winapi::shared::minwindef::HINSTANCE,
    _fdw_reason: winapi::shared::minwindef::DWORD,
    _lpv_reserved: winapi::shared::minwindef::LPVOID,
) -> i32 {
    1
}
