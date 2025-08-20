#![no_main]
#![no_std]
#![allow(non_upper_case_globals)]

#[cfg(feature = "beacon_api")]
mod beacon;
#[cfg(feature = "beacon_api")]
mod beacon_api;
mod bof;
#[cfg(feature = "utils")]
mod utils;
#[cfg(feature = "utils")]
mod winapi;
#[panic_handler]
#[unsafe(no_mangle)]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(feature = "alloc")]
mod allocator;
