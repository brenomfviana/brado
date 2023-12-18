extern crate libc;

use libc::c_char;
use std::ffi::{CStr};
use brado;

#[no_mangle]
pub extern "C" fn validate_str(
    document: *const c_char,
    is_masked: bool,
    ignore_repeated: bool,
) -> bool {
    let s = unsafe {
         match CStr::from_ptr(document).to_str() {
            Ok(v) => v,
            _ => &"",
        }
    };
    brado::cpf::validate_str(s, is_masked, ignore_repeated)
}
