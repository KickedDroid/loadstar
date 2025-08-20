use core::ffi::{c_char, c_int, c_void};
use utils::output;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_bof(args: *mut c_char, alen: c_int) {
    output("HELOOOOOO");
}
