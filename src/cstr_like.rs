use std::ffi::{CStr, c_char};

/// A common interface for C-compatible string types used in this crate.
///
/// `CStrLike` abstracts over different string storage strategies
/// (e.g. stack-allocated and heap-allocated C strings) so they can
/// be used interchangeably.
///
/// Types that implement this trait guarantee:
/// - The returned pointer from [`as_ptr`] is a valid, null-terminated
///   C string for as long as the implementor is alive.
/// - The returned [`CStr`] reference from [`as_cstr`] is always valid.
///
/// This trait is mainly intended to unify [`CStrHeap`] (heap-allocated)
/// and `CStrStack<N>` (stack-allocated with a fixed buffer).
///
/// # Examples
///
/// ```
/// use std::ffi::{CString, CStr};
/// use stack_cstr::{CStrHeap, CStrLike};
///
/// let cstr = CString::new("hello").unwrap();
/// let heap = CStrHeap::new(cstr);
///
/// // Use the trait methods
/// let ptr = heap.as_ptr();
/// let slice: &CStr = heap.as_cstr();
///
/// assert_eq!(slice.to_str().unwrap(), "hello");
/// unsafe {
///     assert_eq!(CStr::from_ptr(ptr).to_str().unwrap(), "hello");
/// }
/// ```
pub trait CStrLike {
    /// Returns a raw pointer to the null-terminated C string.
    ///
    /// The pointer is valid as long as `self` is alive.  
    /// This is mainly intended for FFI calls.
    fn as_ptr(&self) -> *const c_char;

    /// Returns a reference to the underlying [`CStr`].
    fn as_cstr(&self) -> &CStr;
}
