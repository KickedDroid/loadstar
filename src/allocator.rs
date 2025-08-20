use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use utils::{output, resolve_func};
struct WindowsAllocator;

unsafe impl GlobalAlloc for WindowsAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let heap_handle = GetProcessHeap();
        HeapAlloc(heap_handle, 0, layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let heap_handle = GetProcessHeap();
        HeapFree(heap_handle, 0, ptr as *const _);
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: WindowsAllocator = WindowsAllocator;

type GetProcessHeap = unsafe extern "system" fn() -> *const c_void;
pub fn GetProcessHeap() -> *const c_void {
    let some_func = resolve_func("kernel32.dll\0", "GetProcessHeap\0");
    if some_func.is_null() {
        output("Error: GetProccessHeap not found");
        return core::ptr::null_mut();
    }
    let get_heap: GetProcessHeap = unsafe { core::mem::transmute(some_func) };
    unsafe { (get_heap)() }
}

type HeapAlloc =
    unsafe extern "system" fn(handle: *const c_void, dwflags: u32, dwbytes: usize) -> *mut c_void;
pub fn HeapAlloc(handle: *const c_void, dwflags: u32, dwbytes: usize) -> *mut c_void {
    let some_func = resolve_func("kernel32.dll\0", "HeapAlloc\0");
    if some_func.is_null() {
        output("Error: HeapAlloc not found");
        return core::ptr::null_mut();
    }
    let heap_alloc: HeapAlloc = unsafe { core::mem::transmute(some_func) };
    unsafe { (heap_alloc)(handle, dwflags, dwbytes) }
}

type HeapFree =
    unsafe extern "system" fn(hheap: *const c_void, dwflags: u32, lpmem: *const c_void) -> bool;
pub fn HeapFree(hheap: *const c_void, dwflags: u32, lpmem: *const c_void) -> bool {
    let some_func = resolve_func("kernel32.dll\0", "HeapFree\0");
    if some_func.is_null() {
        output("Error: Heap Free Not found");
        return false;
    }
    let heap_free: HeapFree = unsafe { core::mem::transmute(some_func) };
    unsafe { (heap_free)(hheap, dwflags, lpmem) }
}
