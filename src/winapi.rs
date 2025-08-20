use crate::utils::output;
use crate::utils::resolve_func;
use core::ffi::c_void;

#[repr(C)]
pub struct PROCESS_INFORMATION {
    pub hProcess: usize,
    hThread: usize,
    dwProcessId: u32,
    dwThreadId: u32,
}

impl PROCESS_INFORMATION {
    pub fn default() -> Self {
        let mut pi = unsafe { core::mem::zeroed::<PROCESS_INFORMATION>() };
        pi
    }
}

#[repr(C)]
pub struct STARTUPINFO {
    pub cb: u32,
    lpReserved: *mut u8,
    lpDesktop: *mut u8,
    lpTitle: *mut u8,
    dwX: u32,
    dwY: u32,
    dwXSize: u32,
    dwYSize: u32,
    dwXCountChars: u32,
    dwYCountChars: u32,
    dwFillAttribute: u32,
    pub dwFlags: u32,
    pub wShowWindow: u16,
    cbReserved2: u16,
    lpReserved2: *mut u8,
    hStdInput: usize,
    hStdOutput: usize,
    hStdError: usize,
}

impl STARTUPINFO {
    pub fn default() -> Self {
        let mut si = unsafe { core::mem::zeroed::<STARTUPINFO>() };
        si.cb = core::mem::size_of::<STARTUPINFO>() as u32;
        si
    }
}

type CreateProcessW = unsafe extern "system" fn(
    *const u16,
    *mut u16,
    *mut c_void,
    *mut c_void,
    i32,
    u32,
    *mut c_void,
    *const u16,
    *mut STARTUPINFO,
    *mut PROCESS_INFORMATION,
) -> i32;
pub fn CreateProcessW(
    name: *const u16,
    command_line: *mut u16,
    process_attrs: *mut c_void,
    thread_attrs: *mut c_void,
    inherit_handles: i32,
    creation_flags: u32,
    environment: *mut c_void,
    current_dir: *const u16,
    startup_info: *mut STARTUPINFO,
    process_info: *mut PROCESS_INFORMATION,
) -> i32 {
    let some_func = resolve_func("kernel32.dll\0", "CreateProcessW\0");
    if some_func.is_null() {
        output("Error: Not found");
        return 0;
    }
    let create_process: CreateProcessW = unsafe { core::mem::transmute(some_func) };
    unsafe {
        (create_process)(
            name,
            command_line,
            process_attrs,
            thread_attrs,
            inherit_handles,
            creation_flags,
            environment,
            current_dir,
            startup_info,
            process_info,
        )
    }
}

type WaitForSingleObject = unsafe extern "system" fn(usize, u32) -> u32;
pub fn WaitForSingleObject(process: usize, idk: u32) -> u32 {
    let some_func = resolve_func("kernel32.dll\0", "WaitForSingleObject\0");
    if some_func.is_null() {
        output("Error: Not found");
        return 0;
    }

    let wfso: WaitForSingleObject = unsafe { core::mem::transmute(some_func) };
    unsafe { (wfso)(process, idk) }
}

type MessageBoxA = unsafe extern "system" fn(
    hwnd: *mut c_void,
    test: *mut u8,
    caption: *mut u8,
    style: u32,
) -> isize;
pub fn MessageBoxA(wnd: *mut c_void, test: *mut u8, caption: *mut u8, style: u32) -> isize {
    let some_func = resolve_func("user32.dll\0", "MessageBoxA\0");
    if some_func.is_null() {
        output("Error: Not found");
        return 0;
    }
    let msgbox: MessageBoxA = unsafe { core::mem::transmute(some_func) };
    unsafe { (msgbox)(wnd, test, caption, style) }
}

type VirtualAlloc = unsafe extern "system" fn(
    lpaddress: *const c_void,
    dwsize: usize,
    flallocationtype: u32,
    flprotect: u32,
) -> *mut c_void;
pub fn VirtualAlloc(
    lpaddress: *const c_void,
    dwsize: usize,
    flallocationtype: u32,
    flprotect: u32,
) -> *mut c_void {
    let vfunc_addr = resolve_func("kernel32.dll\0", "VirtualAlloc\0");
    if vfunc_addr.is_null() {
        output("Error: Not found, VirtualAlloc");
        return core::ptr::null_mut();
    }
    let virtual_alloc: VirtualAlloc = unsafe { core::mem::transmute(vfunc_addr) };

    unsafe { (virtual_alloc)(lpaddress, dwsize, flallocationtype, flprotect) }
}
