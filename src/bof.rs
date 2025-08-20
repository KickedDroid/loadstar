use core::ffi::{c_char, c_int, c_void};
use utils::resolve_func;
use utils::{output, printf, GetProcAddress, LoadLibraryA};

type MessageBoxA = unsafe extern "system" fn(
    hwnd: *mut c_void,
    test: *mut u8,
    caption: *mut u8,
    style: u32,
) -> isize;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_bof(args: *mut c_char, alen: c_int) {
    output("HELOOOOOO");
}
