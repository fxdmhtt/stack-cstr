/// A macro to create a C-compatible string (`&CStr`) with stack allocation fallback.
///
/// The `cstr!` macro constructs a [`CArrayString`] with a default internal stack buffer.
/// If the string does not fit in the stack buffer, it automatically falls back to
/// allocating a [`CString`] on the heap.
///
/// This makes it ergonomic to create FFI-safe strings with minimal overhead.
/// Short strings will typically stay on the stack, while longer strings automatically
/// use the heap.
///
/// # Syntax
///
/// ```ignore
/// cstr!("format string", args...)   // uses default stack size 128
/// ```
///
/// - `"format string", args...`: A format string and its arguments, just like in `format!`.
///
/// # Returns
///
/// A `CArrayString<128>`, which can be used to obtain:
/// - a raw pointer (`*const c_char`) via [`CArrayString::as_ptr`]
/// - a reference to the [`CStr`] via [`CArrayString::as_c_str`]
///
/// # Examples
///
/// ```
/// use std::ffi::CStr;
/// 
/// use stack_cstr::cstr;
///
/// let s = cstr!("Pi = {:.2}", 3.14159);
/// assert_eq!(s.as_c_str().to_str().unwrap(), "Pi = 3.14");
///
/// unsafe {
///     // Pass to FFI as *const c_char
///     let ptr = s.as_ptr();
///     assert_eq!(CStr::from_ptr(ptr).to_str().unwrap(), "Pi = 3.14");
/// }
/// ```
///
/// # Notes
///
/// - The macro uses a default stack buffer of 128 bytes. Strings that fit in this buffer
///   do not require heap allocation.
/// - If the formatted string is too long, the macro falls back to a heap allocation internally.
/// - The returned type is `CArrayString<128>`. The actual storage may be stack- or heap-based
///   depending on the string length.
///
/// # See also
///
/// - [`CArrayString`] for more control over stack/heap allocation.
#[macro_export]
macro_rules! cstr {
    ( $($args:tt)* ) => {
        $crate::CArrayString::<128>::new(format_args!($($args)*))
    };
}
