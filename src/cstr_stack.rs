use std::ffi::{CStr, c_char};

use arrayvec::ArrayString;

use crate::{CStrError, CStrLike};

/// A stack-allocated, null-terminated C string with fixed capacity.
///
/// `CStrStack<N>` stores a formatted string directly on the stack, avoiding heap allocation.
/// The internal buffer has `N` bytes and is always null-terminated (`\0`),
/// making it safe to pass to C functions via `as_ptr()`.
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
/// - Returns `Err("buffer overflow")` if the formatted string (plus NUL) does not fit in the buffer.
/// - Returns `Err("format failed")` if formatting fails (rare case; ArrayString rarely errors).
pub struct CStrStack<const N: usize> {
    buf: ArrayString<N>,
}

impl<const N: usize> CStrStack<N> {
    /// Creates a new stack-allocated C string using a `format_args!` expression.
    ///
    /// The string is written into an internal buffer of size `N`.
    /// If the string does not fit, returns an error.
    pub fn new(fmt: std::fmt::Arguments) -> Result<CStrStack<N>, CStrError> {
        let mut buf: ArrayString<N> = ArrayString::new();
        std::fmt::write(&mut buf, fmt)?;

        buf.try_push('\0')?;

        Ok(Self { buf })
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
        unsafe { CStr::from_bytes_with_nul_unchecked(&self.buf.as_bytes()[..self.buf.len()]) }
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
