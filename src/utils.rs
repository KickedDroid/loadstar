use core::ffi::{c_char, c_int, c_void};

unsafe extern "C" {
    fn starOutput(r#type: c_int, data: *const c_char, len: c_int);
    fn starToWideChar(src: *const c_char, dst: *mut u16, max: c_int) -> bool;
    fn starPrintf(r#type: c_int, str: *const c_char, ...);
}

pub fn output(bytes: &str) {
    unsafe {
        (starOutput)(
            0 as c_int,
            bytes.as_ptr() as *const c_char,
            bytes.len() as i32,
        );
    }
    unsafe {
        (starOutput)(0 as c_int, core::ptr::null_mut() as *const c_char, 0 as i32);
    }
}

pub fn printf(str: &str, arg: *const c_char) {
    unsafe {
        starPrintf(0, str.as_ptr() as *const c_char, arg as *const c_char);
    }
}

pub fn to_wide(src: &str, dst: *mut u16) -> bool {
    let max = src.len() + 1;

    unsafe { starToWideChar(src.as_ptr() as *const c_char, dst, max as c_int) }
}

unsafe extern "C" {
    fn starLoadLibraryA(libname: *const u8) -> *mut c_void;
    fn starGetProcAddress(handle: *mut c_void, procname: *const u8) -> *mut c_void;
}

pub fn LoadLibraryA(libname: &str) -> *mut c_void {
    unsafe { starLoadLibraryA(libname.as_ptr() as *const u8) }
}

pub fn GetProcAddress(handle: *mut c_void, procname: &str) -> *mut c_void {
    let fn_handle = unsafe { starGetProcAddress(handle, procname.as_ptr() as *const u8) };

    return fn_handle;
}

pub fn resolve_func(lib: &str, func: &str) -> *const c_void {
    let handle = LoadLibraryA(lib);
    if handle.is_null() {
        output("[!] Failed to find dll");
        return core::ptr::null_mut();
    }
    GetProcAddress(handle, func)
}
