use std::ffi::{CStr, CString, c_char};

use arrayvec::ArrayString;

use crate::{CStrError, ContainsNulError};

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum CArrayString<const N: usize> {
    Stack(ArrayString<N>),
    Heap(CString),
}

impl<const N: usize> From<&CStr> for CArrayString<N> {
    fn from(value: &CStr) -> Self {
        if value.count_bytes() < N {
            let mut buf = ArrayString::<N>::new();
            buf.push_str(unsafe { str::from_utf8_unchecked(value.to_bytes()) });
            buf.push('\0');
            Self::Stack(buf)
        } else {
            Self::Heap(value.to_owned())
        }
    }
}

impl<const N: usize> TryFrom<&[u8]> for CArrayString<N> {
    type Error = CStrError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        CStr::from_bytes_with_nul(value)
            .map(CArrayString::from)
            .map_err(Into::into)
    }
}

impl<const N: usize> From<&CString> for CArrayString<N> {
    fn from(value: &CString) -> Self {
        From::<&CStr>::from(value)
    }
}

impl<const N: usize> From<CString> for CArrayString<N> {
    fn from(value: CString) -> Self {
        Self::Heap(value)
    }
}

impl<const N: usize> TryFrom<&str> for CArrayString<N> {
    type Error = CStrError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() < N {
            let bytes = value.as_bytes();
            match core::slice::memchr::memchr(0, bytes) {
                Some(_i) => Err(Into::into(ContainsNulError)),
                None => Ok({
                    let mut buf = ArrayString::<N>::new();
                    buf.push_str(value);
                    buf.push('\0');
                    Self::Stack(buf)
                }),
            }
        } else {
            CString::new(value).map(Self::Heap).map_err(Into::into)
        }
    }
}

impl<const N: usize> TryFrom<&String> for CArrayString<N> {
    type Error = CStrError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        TryFrom::<&str>::try_from(value)
    }
}

/// A C-compatible string type that can be stored on the stack or heap.
///
/// `CArrayString<N>` provides a unified abstraction over two storage strategies:
///
/// 1. **Stack-allocated:** Uses [`ArrayString<N>`] for small strings that fit into
///    a fixed-size buffer. This avoids heap allocation and is very efficient.
/// 2. **Heap-allocated:** Uses [`CString`] when the string exceeds the stack buffer,
///    ensuring the string is always valid and null-terminated.
///
/// This type guarantees:
/// - [`as_ptr`] always returns a valid, null-terminated C string pointer for the lifetime of `self`.
/// - [`as_c_str`] always returns a valid [`CStr`] reference.
///
/// # Stack vs Heap Behavior
///
/// When creating a `CArrayString` via [`new`], the string is first attempted to be stored on
/// the stack. If it does not fit, it falls back to a heap allocation:
///
/// ```text
/// ┌───────────────┐
/// │ Stack Buffer  │  (ArrayString<N>)
/// └───────────────┘
///       │ fits
///       └─> use stack
///
///       │ does not fit
///       └─> allocate heap (CString)
/// ```
///
/// # Performance Considerations
///
/// - Small strings that fit in the stack buffer avoid heap allocations and are faster.
/// - Large strings trigger heap allocation, which may be slower and use more memory.
/// - Prefer choosing `N` large enough for your common use case to minimize heap fallbacks.
///
/// # Examples
///
/// ```
/// use std::ffi::CStr;
/// 
/// use stack_cstr::CArrayString;
///
/// // Small string fits on stack
/// let stack_str = CArrayString::<16>::new(format_args!("hello"));
/// assert!(matches!(stack_str, CArrayString::Stack(_)));
///
/// // Large string falls back to heap
/// let heap_str = CArrayString::<4>::new(format_args!("this is too long"));
/// assert!(matches!(heap_str, CArrayString::Heap(_)));
///
/// // Accessing as CStr
/// let cstr: &CStr = heap_str.as_c_str();
/// assert_eq!(cstr.to_str().unwrap(), "this is too long");
///
/// // Raw pointer for FFI
/// let ptr = stack_str.as_ptr();
/// unsafe {
///     assert_eq!(CStr::from_ptr(ptr).to_str().unwrap(), "hello");
/// }
/// ```
impl<const N: usize> CArrayString<N> {
    /// Creates a new C-compatible string using `format_args!`.
    ///
    /// Attempts to store the formatted string in a stack buffer of size `N`.
    /// Falls back to a heap allocation if the string does not fit.
    ///
    /// # Parameters
    ///
    /// - `fmt`: The formatted arguments, typically produced by `format_args!`.
    ///
    /// # Returns
    ///
    /// A `CArrayString<N>` containing the formatted string.
    ///
    /// # Notes
    ///
    /// - If the stack buffer overflows or writing fails, the string is stored on the heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use stack_cstr::CArrayString;
    ///
    /// let s = CArrayString::<8>::new(format_args!("hi {}!", "you"));
    /// assert!(s.as_c_str().to_str().unwrap().starts_with("hi"));
    /// ```
    pub fn new(fmt: std::fmt::Arguments) -> CArrayString<N> {
        fn try_stack<const N: usize>(
            fmt: std::fmt::Arguments,
        ) -> Result<ArrayString<N>, CStrError> {
            let mut buf: ArrayString<N> = ArrayString::new();
            std::fmt::write(&mut buf, fmt)?;
            buf.try_push('\0')?;
            Ok(buf)
        }

        match try_stack::<N>(fmt) {
            Ok(arr) => Self::Stack(arr),
            Err(_) => Self::Heap(CString::new(std::fmt::format(fmt)).unwrap()),
        }
    }

    /// Returns a raw pointer to the null-terminated C string.
    ///
    /// The pointer is valid for the lifetime of `self`.
    /// This is useful for passing the string to C APIs via FFI.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::CStr;
    /// 
    /// use stack_cstr::CArrayString;
    ///
    /// let s = CArrayString::<8>::new(format_args!("hello"));
    /// let ptr = s.as_ptr();
    /// unsafe {
    ///     assert_eq!(CStr::from_ptr(ptr).to_str().unwrap(), "hello");
    /// }
    /// ```
    pub fn as_ptr(&self) -> *const c_char {
        match self {
            CArrayString::Stack(s) => s.as_ptr() as _,
            CArrayString::Heap(s) => s.as_ptr(),
        }
    }

    /// Returns a reference to the underlying [`CStr`].
    ///
    /// Provides safe access to the string as a `&CStr` without exposing the
    /// underlying storage strategy.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::CStr;
    /// 
    /// use stack_cstr::CArrayString;
    ///
    /// let s = CArrayString::<8>::new(format_args!("hello"));
    /// let cstr: &CStr = s.as_c_str();
    /// assert_eq!(cstr.to_str().unwrap(), "hello");
    /// ```
    pub fn as_c_str(&self) -> &CStr {
        match self {
            CArrayString::Stack(s) => unsafe { CStr::from_bytes_with_nul_unchecked(s.as_bytes()) },
            CArrayString::Heap(s) => s.as_c_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_overflow() {
        assert_eq!(
            CArrayString::<12>::try_from("hello world")
                .unwrap()
                .as_c_str()
                .to_str()
                .unwrap(),
            "hello world"
        );
        assert_eq!(
            CArrayString::<11>::try_from("hello world")
                .unwrap()
                .as_c_str()
                .to_str()
                .unwrap(),
            "hello world"
        );
    }

    #[test]
    fn test_cstr() {
        assert_eq!(
            CArrayString::<12>::from(c"hello world")
                .as_c_str()
                .to_str()
                .unwrap(),
            "hello world"
        );
        assert_eq!(
            CArrayString::<11>::from(c"hello world")
                .as_c_str()
                .to_str()
                .unwrap(),
            "hello world"
        );
    }

    #[test]
    fn test_format_args() {
        let s1 = "hello";
        let s2 = "world";
        assert_eq!(
            CArrayString::<12>::new(format_args!("{s1} world"))
                .as_c_str()
                .to_str()
                .unwrap(),
            "hello world"
        );
        assert_eq!(
            CArrayString::<11>::new(format_args!("hello {s2}"))
                .as_c_str()
                .to_str()
                .unwrap(),
            "hello world"
        );
    }
}
