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
    let some_func = resolve_func("user32.dll\0", "MessageBoxA\0");
    if !some_func.is_null() {
        let msgbox: MessageBoxA = unsafe { core::mem::transmute(some_func) };

        match (msgbox)(
            core::ptr::null_mut(),
            b"Hello bruh\0".as_ptr() as *mut u8,
            b"Rusty BOF\0".as_ptr() as *mut u8,
            1,
        ) as i32
        {
            1 => output("you clicked ok"),
            2 => output("You clicked cancel"),
            _ => output("exited"),
        }
    } else {
        output("[!] Could not find func");
    }
}
