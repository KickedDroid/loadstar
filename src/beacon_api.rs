#![allow(non_upper_case_globals)]
#[warn(unused_imports)]
use core::ffi::{c_char, c_int, c_short, c_void};
#[repr(C, align(8))]
pub struct FormatP {
    pub original: *mut c_char,
    pub buffer: *mut c_char,
    pub length: c_int,
    pub size: c_int,
}

impl FormatP {
    pub fn new() -> Self {
        FormatP {
            original: core::ptr::null_mut(),
            buffer: core::ptr::null_mut(),
            length: 0,
            size: 0,
        }
    }
}

#[repr(C)]
pub struct DataP {
    pub original: *mut c_char,
    pub buffer: *mut c_char,
    pub length: c_int,
    pub size: c_int,
}

impl DataP {
    pub fn new() -> Self {
        DataP {
            original: core::ptr::null_mut(),
            buffer: core::ptr::null_mut(),
            length: 0,
            size: 0,
        }
    }
}

unsafe extern "C" {
    fn starFormatAlloc(formatp: *mut FormatP, maxsz: c_int);
    fn starFormatFree(formatp: *mut FormatP);
    fn starFormatToString(formatp: *mut FormatP, size: c_int) -> *const c_char;
    fn starFormatAppend(formatp: *mut FormatP, text: *const c_char, len: c_int);
    fn starFormatReset(formatp: *mut FormatP);
    fn starDataParse(datap: *mut DataP, args: *mut c_char, alen: c_int);
    fn starDataExtract(datap: *mut DataP, size: *mut c_int) -> *mut c_char;
    fn starDataInt(datap: *mut DataP) -> i32;
}

pub fn FormatAlloc(formatp: *mut FormatP, maxsz: c_int) {
    unsafe {
        starFormatAlloc(formatp, maxsz);
    }
}

pub fn FormatFree(formatp: *mut FormatP) {
    unsafe {
        starFormatFree(formatp);
    }
}

pub fn FormatToString(formatp: *mut FormatP, size: c_int) -> *const c_char {
    unsafe { starFormatToString(formatp, size) }
}

pub fn FormatAppend(formatp: *mut FormatP, text: *const c_char, len: i32) {
    unsafe { starFormatAppend(formatp, text, len) }
}

pub fn FormatReset(formatp: *mut FormatP) {
    unsafe { starFormatReset(formatp) }
}

pub fn DataParse(datap: *mut DataP, args: *mut c_char, alen: c_int) {
    unsafe {
        starDataParse(datap, args, alen);
    }
}

pub fn DataExtract(parser: *mut DataP, alen: c_int) -> *mut c_char {
    //let mut parser = DataP::new();
    let mut size: c_int = 0;
    unsafe { starDataExtract(parser, size as *mut i32) }
}

pub fn DataInt(datap: *mut DataP) -> i32 {
    unsafe { starDataInt(datap) }
}

/*
unsafe extern "C" {
    fn starVirtualAlloc(
        lpaddress: *const c_void,
        dwsize: usize,
        flallocationtype: u32,
        flprotect: u32,
    ) -> *mut c_void;
}
*/
