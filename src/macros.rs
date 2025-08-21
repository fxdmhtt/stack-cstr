/// A macro to create a C-compatible string (`&CStr`) with stack allocation fallback.
///
/// The `cstr!` macro tries to construct a [`CStrStack`] with one of the given
/// buffer sizes. If none of the stack sizes is large enough, it falls back to
/// allocating a [`CString`] on the heap.
///
/// This makes it ergonomic to create FFI-safe strings with minimal overhead.
/// Common short strings will use stack buffers, while longer strings
/// automatically use heap allocation.
///
/// # Syntax
///
/// ```ignore
/// cstr!([sizes...], "format string", args...)
/// cstr!("format string", args...)   // uses default sizes [32, 128]
/// ```
///
/// - `sizes...`: A list of candidate stack buffer sizes.  
///   The macro tries each size in order until one succeeds.  
///   If all fail, a heap allocation is used.  
/// - `"format string", args...`: A format string and its arguments,
///   just like in `format!`.
///
/// # Returns
///
/// A `Box<dyn CStrLike>`, which can be used to obtain:
/// - a raw pointer (`*const c_char`) via [`CStrLike::as_ptr`]
/// - a reference to the [`CStr`] via [`CStrLike::as_cstr`]
///
/// # Examples
///
/// ```
/// use std::ffi::CStr;
/// use stack_cstr::{cstr, CStrLike};
///
/// // Use default sizes [32, 128]
/// let s = cstr!("Hello {}", 42);
/// assert_eq!(s.as_cstr().to_str().unwrap(), "Hello 42");
///
/// // Explicit stack sizes
/// let s = cstr!([16, 64], "Pi = {:.2}", 3.14159);
/// assert_eq!(s.as_cstr().to_str().unwrap(), "Pi = 3.14");
///
/// unsafe {
///     // Pass to FFI as *const c_char
///     assert_eq!(CStr::from_ptr(s.as_ptr()).to_str().unwrap(), "Pi = 3.14");
/// }
/// ```
///
/// # Notes
///
/// - If the formatted string fits in one of the provided stack buffers,
///   no heap allocation is performed.  
/// - If the string is too long for all stack buffers, it is allocated on the heap.
/// - The returned type is `Box<dyn CStrLike>`, which is heap-allocated for
///   type erasure even when the string is stack-based. This indirection is
///   usually negligible.
///
/// # See also
///
/// - [`CStrStack`] for stack-only storage
/// - [`CStrHeap`] for explicit heap allocation
#[macro_export]
macro_rules! cstr {
    ([$($size:expr),*], $($args:tt)*) => {{
        let args = format_args!($($args)*);

        let result: Box<dyn $crate::CStrLike> = if false { unreachable!() }
        $(
            else if let Ok(s) = $crate::CStrStack::<$size>::new(args) {
                Box::new(s)
            }
        )*
        else {
            Box::new($crate::CStrHeap::new(std::ffi::CString::new(format!($($args)*)).unwrap()))
        };

        result
    }};
    ($($args:tt)*) => {
        cstr!([32, 128], $($args)*)
    };
}
