use beacon_api::FormatP;
use beacon_api::{DataExtract, DataInt, DataP, DataParse, FormatAlloc, FormatFree};
use core::ffi::{c_char, c_int};
pub struct Beacon {
    pub format: FormatP,
    pub data: DataP,
    pub args: *const c_char,
    pub alen: c_int,
}
impl Beacon {
    pub fn new(args: *const c_char, alen: c_int) -> Self {
        let mut beacon = Beacon {
            format: FormatP::new(),
            data: DataP::new(),
            args,
            alen,
        };

        DataParse(&mut beacon.data as *mut DataP, args as *mut c_char, alen);
        return beacon;
    }

    pub fn format_alloc(&mut self, size: c_int) {
        FormatAlloc(&mut self.format as *mut FormatP, size)
    }

    pub fn get_arg(&mut self) -> *mut c_char {
        DataExtract(&mut self.data, self.alen)
    }

    pub fn get_int(&mut self) -> i32 {
        DataInt(&mut self.data)
    }

    pub fn free(&mut self) {
        FormatFree(&mut self.format)
    }
}

impl Drop for Beacon {
    fn drop(&mut self) {
        self.free();
    }
}
