use std::ffi::{CStr, c_char};

use arrayvec::ArrayString;

use crate::CStrLike;

pub struct CStrStack<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> CStrStack<N> {
    pub fn new(fmt: std::fmt::Arguments) -> Result<CStrStack<N>, &'static str> {
        let mut buf: ArrayString<N> = ArrayString::new();
        std::fmt::write(&mut buf, fmt).map_err(|_| "format failed")?;

        let bytes = buf.as_bytes();
        if bytes.len() + 1 > N {
            return Err("stack buffer overflow");
        }

        let mut c_buf: [u8; N] = [0; N];
        c_buf[..bytes.len()].copy_from_slice(bytes);
        c_buf[bytes.len()] = 0;

        Ok(CStrStack {
            buf: c_buf,
            len: bytes.len(),
        })
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.buf.as_ptr() as *const c_char
    }

    pub fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(&self.buf[..self.len + 1]) }
    }
}

impl<const N: usize> CStrLike for CStrStack<N> {
    fn as_ptr(&self) -> *const c_char {
        self.as_ptr()
    }

    fn as_cstr(&self) -> &CStr {
        self.as_cstr()
    }
}
