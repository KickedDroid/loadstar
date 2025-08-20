use beacon::Beacon;
use bindings::{output, printf};
use core::ffi::{c_char, c_int};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_bof(args: *mut c_char, alen: c_int) {
    let mut beacon = Beacon::new(args, alen);
    beacon.format_alloc(256);

    let ip = beacon.get_arg();
    if !ip.is_null() {
        printf(b"Ip: %s \0", ip);
        let port = beacon.get_int();
        printf(b"Port: %d \0", port as *mut i8);
    } else {
        output(b"Usage: str:ip int:8080 \0");
    }
}
