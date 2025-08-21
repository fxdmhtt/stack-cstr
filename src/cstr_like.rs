use std::ffi::{CStr, c_char};

pub trait CStrLike {
    fn as_ptr(&self) -> *const c_char;
    fn as_cstr(&self) -> &CStr;
}
