extern crate libc;

use libc::c_char;
use std::ffi::{CStr};
use brado;

#[no_mangle]
pub extern "C" fn cnpj_validate_str(
    document: *const c_char,
    is_masked: bool,
) -> bool {
    let s = unsafe {
         match CStr::from_ptr(document).to_str() {
            Ok(v) => v,
            _ => &"",
        }
    };
    brado::cnpj::validate_str(s, is_masked)
}
