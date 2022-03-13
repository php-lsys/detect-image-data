
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_double;
use std::os::raw::c_uint;

#[repr(C)]
pub struct ZxingCResult {
    pub status: c_int,
    pub num_bits: c_int,
    pub format: c_int,
    pub orientation:c_double,
    pub line_count:c_int,
    pub bytes: *mut u8,
    pub bytes_size: c_int,
    pub corners: [c_int; 8],
    pub width: c_uint,
    pub height: c_uint,
    pub next: *const ZxingCResult,
}

#[link(name = "zxing_c_api", kind = "static")]
extern "C" {
    pub  fn zxing_read_qrcode(
        out_result : *mut *mut ZxingCResult,
        out_width: *mut c_int,
        out_height: *mut c_int,
        buffer: *mut c_char,
        fast: c_int,
        norotate: c_int,
        ispure: c_int,
        desired_channels:c_int,
    )-> c_int;
    pub fn zxing_release_result(result: *mut ZxingCResult)-> c_int;
}