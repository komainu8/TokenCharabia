Use std::os::raw::c_char;
use std::ffi::CStr;

use charabia::Tokenize;

#[no_mangle]
pub extern "C" fn tokenize(document_raw: *const c_char) {
    let document = unsafe { CStr::from_ptr(document_raw).to_str().unwrap() };
    let mut tokens = document.tokenize();
}
