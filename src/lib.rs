//! # stack_cstr
//!
//! `stack_cstr` provides ergonomic and efficient ways to create
//! [`CStr`](std::ffi::CStr) values for FFI interoperability.
//!
//! The crate offers both **stack-based** and **heap-based** strategies
//! for storing C-compatible strings. Its goal is to minimize heap
//! allocations for short-lived or short strings while providing
//! automatic fallback to the heap when needed.
//!
//! ## Motivation
//!
//! When interacting with C APIs, strings must be passed as
//! null-terminated `*const c_char`. The standard way in Rust
//! is to use [`CString`], which always allocates on the heap.
//!
//! However, in performance-sensitive or embedded environments,
//! frequent heap allocations are undesirable. `stack_cstr` allows
//! you to create `CStr` objects backed by **fixed-size stack buffers**,
//! avoiding heap allocations for common short strings.
//!
//! ## Core Components
//!
//! - [`CStrLike`](crate::CStrLike): A trait for types that can expose
//!   a `*const c_char` and a `&CStr`.
//! - [`CStrStack`](crate::CStrStack): A stack-allocated C string with
//!   a user-defined capacity.
//! - [`CStrHeap`](crate::CStrHeap): A heap-allocated C string wrapper
//!   around [`CString`].
//! - [`cstr!`](crate::cstr): A macro that automatically chooses between
//!   stack or heap storage depending on the string length.
//!
//! ## Example: Using the `cstr!` Macro
//!
//! ```
//! use std::ffi::CStr;
//! use stack_cstr::{cstr, CStrLike};
//!
//! // Default stack sizes are [32, 128]
//! let s = cstr!("Hello {}", 42);
//! assert_eq!(s.as_cstr().to_str().unwrap(), "Hello 42");
//!
//! // Explicitly set candidate stack buffer sizes
//! let s = cstr!([16, 64], "Pi = {:.2}", 3.14159);
//! assert_eq!(s.as_cstr().to_str().unwrap(), "Pi = 3.14");
//!
//! unsafe {
//!     // Pass to FFI as *const c_char
//!     let ptr = s.as_ptr();
//!     assert_eq!(CStr::from_ptr(ptr).to_str().unwrap(), "Pi = 3.14");
//! }
//! ```
//!
//! ## Example: Manual Use of `CStrStack`
//!
//! ```
//! use stack_cstr::CStrStack;
//!
//! // Create a stack-allocated C string with capacity 32
//! let s = CStrStack::<32>::new(format_args!("ABC {}!", 123)).unwrap();
//! assert_eq!(s.as_cstr().to_str().unwrap(), "ABC 123!");
//! ```
//!
//! ## Example: Manual Use of `CStrHeap`
//!
//! ```
//! use std::ffi::CString;
//! use stack_cstr::CStrHeap;
//!
//! let heap_str = CStrHeap::new(CString::new("Hello from heap").unwrap());
//! assert_eq!(heap_str.as_cstr().to_str().unwrap(), "Hello from heap");
//! ```
//!
//! ## Design Notes
//!
//! - `CStrStack` checks buffer boundaries at construction time and
//!   guarantees proper null termination.
//! - `cstr!` hides the complexity by trying multiple stack sizes and
//!   falling back to heap when necessary.
//! - Returned values from `cstr!` are `Box<dyn CStrLike>` for uniformity,
//!   even when stack-backed. This introduces a single heap allocation
//!   for type erasure, which is usually negligible compared to string allocation.
//!
//! ## Use Cases
//!
//! - Embedding short strings in FFI calls without heap allocation.
//! - Performance-sensitive applications where allocation patterns matter.
//! - Embedded systems where heap memory is constrained.
//!
//! ## Limitations
//!
//! - `CStrStack` requires a compile-time constant buffer size.
//! - Even stack-backed strings returned from `cstr!` are wrapped in a `Box`
//!   to allow dynamic dispatch.
//!
//! ## See Also
//!
//! - [`CString`](std::ffi::CString) for heap-allocated C strings
//! - [`CStr`](std::ffi::CStr) for borrowed C strings
//! - [`arrayvec`](https://docs.rs/arrayvec) used internally for formatting

pub mod cstr_heap;
pub mod cstr_like;
pub mod cstr_stack;
pub mod macros;

pub use cstr_heap::CStrHeap;
pub use cstr_like::CStrLike;
pub use cstr_stack::CStrStack;
