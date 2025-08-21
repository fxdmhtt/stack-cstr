use std::ffi::{CStr, CString, c_char};

use crate::CStrLike;

/// A heap-allocated C-compatible string wrapper.
///
/// `CStrHeap` owns a [`CString`] internally, and implements the
/// [`CStrLike`] trait to provide a uniform interface with
/// stack-allocated alternatives (e.g. `CStrStack`).
///
/// This is used as a fallback when the string cannot fit into
/// a fixed-size stack buffer.
pub struct CStrHeap {
    cstr: CString,
}

impl CStrHeap {
    /// Creates a new `CStrHeap` from a given [`CString`].
    ///
    /// # Arguments
    ///
    /// * `s` - A [`CString`] instance to be wrapped.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::CString;
    /// use stack_cstr::CStrHeap;
    ///
    /// let cstr = CString::new("hello").unwrap();
    /// let heap = CStrHeap::new(cstr);
    /// assert_eq!(unsafe { std::ffi::CStr::from_ptr(heap.as_ptr()) }.to_str().unwrap(), "hello");
    /// ```
    pub fn new(s: CString) -> Self {
        Self { cstr: s }
    }

    /// Returns a raw pointer to the underlying C string.
    ///
    /// The pointer is guaranteed to be valid as long as the `CStrHeap`
    /// instance is alive. The string is null-terminated.
    pub fn as_ptr(&self) -> *const c_char {
        self.cstr.as_ptr()
    }

    /// Returns a reference to the underlying [`CStr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::CString;
    /// use stack_cstr::CStrHeap;
    ///
    /// let cstr = CString::new("hello").unwrap();
    /// let heap = CStrHeap::new(cstr);
    /// let slice = heap.as_cstr();
    /// assert_eq!(slice.to_str().unwrap(), "hello");
    /// ```
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
