use std::ffi::{CStr, c_char};

use arrayvec::ArrayString;

use crate::CStrLike;

/// A stack-allocated C string with a fixed buffer size.
///
/// `CStrStack<N>` stores a formatted string directly on the stack
/// with a buffer of `N` bytes, avoiding heap allocation.  
/// It always appends a trailing `\0` (null terminator) so it can be safely
/// passed to C FFI functions.
///
/// If the formatted string (plus the null terminator) does not fit
/// into the buffer, [`new`] will return an error.
///
/// # Examples
///
/// ```
/// use std::ffi::CStr;
/// use stack_cstr::{CStrStack, CStrLike};
///
/// // Create a stack-allocated C string with capacity for 32 bytes
/// let s = CStrStack::<32>::new(format_args!("Hello {}", 123)).unwrap();
///
/// let cstr: &CStr = s.as_cstr();
/// assert_eq!(cstr.to_str().unwrap(), "Hello 123");
///
/// unsafe {
///     // FFI-safe pointer
///     assert_eq!(CStr::from_ptr(s.as_ptr()).to_str().unwrap(), "Hello 123");
/// }
/// ```
///
/// # Errors
///
/// Returns `Err("stack buffer overflow")` if the formatted string is too
/// long to fit in the buffer.
///
/// Returns `Err("format failed")` if formatting the string fails
/// (rare case, usually only if the formatter writes an error).
pub struct CStrStack<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> CStrStack<N> {
    /// Creates a new stack-allocated C string using a `format_args!` expression.
    ///
    /// The string is written into an internal buffer of size `N`.
    /// If the string does not fit, returns an error.
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

    /// Returns a raw pointer to the null-terminated C string.
    ///
    /// This pointer is valid for as long as `self` is alive.
    /// Suitable for passing to FFI.
    pub fn as_ptr(&self) -> *const c_char {
        self.buf.as_ptr() as *const c_char
    }

    /// Returns a reference to the underlying [`CStr`].
    ///
    /// # Safety
    ///
    /// The buffer is guaranteed to be null-terminated by construction,
    /// so this is always safe.
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
