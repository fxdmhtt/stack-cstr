use std::ffi::{CStr, CString, c_char};

use crate::CStrLike;

pub struct CStrHeap {
    cstr: CString,
}

impl CStrHeap {
    pub fn new(s: CString) -> Self {
        Self { cstr: s }
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.cstr.as_ptr()
    }

    pub fn as_cstr(&self) -> &CStr {
        self.cstr.as_c_str()
    }
}

impl CStrLike for CStrHeap {
    fn as_ptr(&self) -> *const c_char {
        self.as_ptr()
    }

    fn as_cstr(&self) -> &CStr {
        self.as_cstr()
    }
}
