use beacon::Beacon;
use bindings::{output, printf, GetProcAddress, LoadLibraryA};
use core::ffi::{c_char, c_int, c_void};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_bof(args: *mut c_char, alen: c_int) {
    let mut beacon = Beacon::new(args, alen);
    beacon.format_alloc(256);
    WSAStartup(2, core::ptr::null_mut() as *mut c_void);
    let socket = WSASocketA(0, 1, 6, core::ptr::null_mut(), 0, 0);
    printf(b"soket: %p\0", socket as *mut i8);
    let res = closesocket(socket);
    printf(b"Output: %d \0", res as *mut i8);

    WSACleanup();
}

fn WSAStartup(wversionrequested: u16, lpwsadata: *mut c_void) -> i32 {
    let handle = LoadLibraryA(b"Ws2_32.dll\0");
    match handle.is_null() {
        true => output(b"WTF\0"),
        false => printf(b"[+] Found .dll at %p \0", handle as *mut i8),
    }

    let some_func = GetProcAddress(handle, b"WSAStartup\0");
    match some_func.is_null() {
        true => output(b"[!] Could not find func\0"),
        false => {
            output(b"[+] Found MessageBoxA func address \0");
            let socket: unsafe extern "system" fn(
                wversionrequested: u16,
                lpwsadata: *mut c_void,
            ) -> i32 = unsafe { core::mem::transmute(some_func) };

            unsafe {
                return (socket)(wversionrequested, lpwsadata);
            }
        }
    }
    0
}

fn WSASocketA(
    af: i32,
    r#type: i32,
    protocol: i32,
    lpprotocolinfo: *const c_void,
    g: u32,
    dwflags: u32,
) -> usize {
    let handle = LoadLibraryA(b"Ws2_32.dll\0");
    match handle.is_null() {
        true => output(b"WTF\0"),
        false => printf(b"[+] Found .dll at %p \0", handle as *mut i8),
    }

    let some_func = GetProcAddress(handle, b"WSASocketA\0");
    match some_func.is_null() {
        true => output(b"[!] Could not find func\0"),
        false => {
            output(b"[+] Found MessageBoxA func address \0");
            let socket: unsafe extern "system" fn(
                af: i32,
                r#type: i32,
                protocol: i32,
                lpprotocolinfo: *const c_void,
                g: u32,
                dwflags: u32,
            ) -> usize = unsafe { core::mem::transmute(some_func) };

            unsafe {
                return (socket)(af, r#type, protocol, lpprotocolinfo, g, dwflags);
            }
        }
    }
    0
}

fn WSACleanup() {
    let handle = LoadLibraryA(b"Ws2_32.dll\0");
    match handle.is_null() {
        true => output(b"WTF\0"),
        false => printf(b"[+] Found .dll at %p \0", handle as *mut i8),
    }

    let some_func = GetProcAddress(handle, b"WSACleanup\0");
    match some_func.is_null() {
        true => output(b"[!] Could not find func\0"),
        false => {
            output(b"[+] Found MessageBoxA func address \0");
            let cleanup: unsafe extern "system" fn() = unsafe { core::mem::transmute(some_func) };

            unsafe { (cleanup)() };
        }
    }
}

fn closesocket(socket: usize) -> i32 {
    let handle = LoadLibraryA(b"Ws2_32.dll\0");
    match handle.is_null() {
        true => output(b"WTF\0"),
        false => printf(b"[+] Found .dll at %p \0", handle as *mut i8),
    }

    let some_func = GetProcAddress(handle, b"closesocket\0");
    match some_func.is_null() {
        true => output(b"[!] Could not find func\0"),
        false => {
            output(b"[+] Found MessageBoxA func address \0");
            let close: unsafe extern "system" fn(usize) -> i32 =
                unsafe { core::mem::transmute(some_func) };

            unsafe {
                return (close)(socket);
            }
        }
    }
    0
}
