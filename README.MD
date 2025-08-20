A different approach to building BOFs in rust. Similar to `bof-oxide` but uses FFI to create wrapper fns.

## Building

prerequisites `just`

```
cargo install just
```

#### utils

Build with utils enabled allowing us to extract data passed to the coff along with `printf`. This is default.

```bash
just bof-utils
```
Example output

```
bof.o:     file format pe-x86-64

SYMBOL TABLE:
[  0](sec  1)(fl 0x00)(ty    0)(scl   3) (nx 1) 0x0000000000000000 .text
AUX scnlen 0x2a nreloc 3 nlnno 0 checksum 0xb8bb3a1e assoc 1 comdat 0
[  2](sec  2)(fl 0x00)(ty    0)(scl   3) (nx 1) 0x0000000000000000 .data
AUX scnlen 0x0 nreloc 0 nlnno 0 checksum 0x0 assoc 2 comdat 0
[  4](sec  6)(fl 0x00)(ty    0)(scl   3) (nx 1) 0x0000000000000000 .xdata
AUX scnlen 0x8 nreloc 0 nlnno 0 checksum 0xfc539d1 assoc 4 comdat 0
[  6](sec  3)(fl 0x00)(ty    0)(scl   3) (nx 1) 0x0000000000000000 .rdata
AUX scnlen 0x9 nreloc 0 nlnno 0 checksum 0x8a0288e7 assoc 5 comdat 0
[  8](sec  5)(fl 0x00)(ty    0)(scl   3) (nx 1) 0x0000000000000000 .pdata
AUX scnlen 0xc nreloc 3 nlnno 0 checksum 0x2a9379e3 assoc 7 comdat 0
[ 10](sec  1)(fl 0x00)(ty   20)(scl   2) (nx 0) 0x0000000000000000 rust_bof
[ 11](sec  1)(fl 0x00)(ty   20)(scl   2) (nx 1) 0x0000000000000030 go
AUX tagndx 0 ttlsiz 0x0 lnnos 0 next 0
[ 13](sec  1)(fl 0x00)(ty   20)(scl   2) (nx 0) 0x0000000000000054 starLoadLibraryA
[ 14](sec  1)(fl 0x00)(ty   20)(scl   2) (nx 0) 0x0000000000000074 starGetProcAddress
[ 15](sec  1)(fl 0x00)(ty   20)(scl   2) (nx 0) 0x000000000000009f starOutput
[ 16](sec  1)(fl 0x00)(ty   20)(scl   2) (nx 0) 0x00000000000000d4 starPrintf
[ 17](sec  1)(fl 0x00)(ty   20)(scl   2) (nx 0) 0x0000000000000108 starToWideChar
[ 18](sec  1)(fl 0x00)(ty    0)(scl   3) (nx 1) 0x0000000000000030 .text
AUX scnlen 0x10d nreloc 6 nlnno 0
[ 20](sec  2)(fl 0x00)(ty    0)(scl   3) (nx 1) 0x0000000000000000 .data
AUX scnlen 0x0 nreloc 0 nlnno 0
[ 22](sec  6)(fl 0x00)(ty    0)(scl   3) (nx 1) 0x0000000000000008 .xdata
AUX scnlen 0x48 nreloc 0 nlnno 0
[ 24](sec  5)(fl 0x00)(ty    0)(scl   3) (nx 1) 0x000000000000000c .pdata
AUX scnlen 0x48 nreloc 18 nlnno 0
[ 26](sec  4)(fl 0x00)(ty    0)(scl   3) (nx 1) 0x0000000000000000 .rdata$zzz
AUX scnlen 0x1d nreloc 0 nlnno 0
[ 28](sec  0)(fl 0x00)(ty    0)(scl   2) (nx 0) 0x0000000000000000 __imp_BeaconOutput
[ 29](sec  0)(fl 0x00)(ty    0)(scl   2) (nx 0) 0x0000000000000000 __imp_toWideChar
[ 30](sec  0)(fl 0x00)(ty    0)(scl   2) (nx 0) 0x0000000000000000 __imp_GetProcAddress
[ 31](sec  0)(fl 0x00)(ty    0)(scl   2) (nx 0) 0x0000000000000000 __imp_BeaconPrintf
[ 32](sec  0)(fl 0x00)(ty    0)(scl   2) (nx 0) 0x0000000000000000 __imp_LoadLibraryA
```

#### beacon_api

```
just bof-beacon_api
```

example feature usage with argument passing

```rust
let mut beacon = Beacon::new(args, alen);
beacon.format_alloc(256);
let ip = beacon.get_arg();
printf(b"Ip: %s \0", ip);
let port = beacon.get_int();
```

#### alloc

Build with the alloc feature enabled.

```bash
just bof-alloc
```

Allows the use of `vec![]`

#### all

All features enabled

```
just bof-all
```

---

### Parse Arguments

```rust
let mut beacon = Beacon::new(args, alen);
    beacon.format_alloc(256);

    let ip = beacon.get_arg();
    if !ip.is_null() {
        printf("Ip: %s \0", ip);
        let port = beacon.get_int();
        printf("Port: %d \0", port as *mut i8);
    } else {
        output("Usage: str:ip int:8080");
}
```

```
just bof-beacon_api
```


### Load functions from DLLs

```rust
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
    let mut beacon = Beacon::new(args, alen);
    beacon.format_alloc(256);

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
```

```
just bof-utils
```
![image](https://github.com/KickedDroid/loadstar/blob/master/examples/Screenshot_2025-08-20_12-37-33.png?raw=true)

### CreateProcessW

```rust
use alloc::vec;
use bindings::output;
use bindings::{to_wide, CreateProcessW, WaitForSingleObject, PROCESS_INFORMATION, STARTUPINFO};
use core::alloc::Layout;
use core::ffi::{c_char, c_int};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_bof(args: *mut c_char, _alen: c_int) {
    let mut si = STARTUPINFO::default();
    si.dwFlags = 0x1;
    let mut pi = PROCESS_INFORMATION::default();

    let mut dst_buffer = vec![0 as *mut u16; 100];

    let cmd = "powershell.exe -c curl -o hello.txt http://192.168.122.1:8000/wassssssup";
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
```

```
just bof-alloc
```
