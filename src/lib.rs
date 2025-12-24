#![allow(internal_features)]
#![feature(slice_internals)]

//! # stack_cstr
//!
//! `stack_cstr` provides ergonomic and efficient ways to create
//! [`CStr`](std::ffi::CStr) values for FFI interoperability.
//!
//! The crate uses [`CArrayString`] to store C-compatible strings. It aims to
//! minimize heap allocations for short strings by using a fixed-size stack buffer,
//! while automatically falling back to heap allocation for longer strings.
//!
//! ## Motivation
//!
//! When interacting with C APIs, strings must be passed as null-terminated
//! `*const c_char`. The standard approach in Rust is to use [`CString`], which
//! always allocates on the heap.
//!
//! In performance-sensitive or embedded environments, frequent heap allocations
//! are undesirable. `stack_cstr` allows you to create `CStr` objects backed
//! by a stack buffer when possible, avoiding heap allocation for short-lived
//! or small strings.
//!
//! ## Core Components
//!
//! - [`CArrayString`](crate::CArrayString): A string type that can be either
//!   stack- or heap-backed, exposing both `*const c_char` and `&CStr`.
//! - [`cstr!`](crate::cstr): A macro that constructs a `CArrayString` with
//!   default stack allocation and falls back to heap if the string is too large.
//!
//! ## Example: Using the `cstr!` Macro
//!
//! ```
//! use std::ffi::CStr;
//! 
//! use stack_cstr::cstr;
//!
//! let s = cstr!("Pi = {:.2}", 3.14159);
//! assert_eq!(s.as_c_str().to_str().unwrap(), "Pi = 3.14");
//!
//! unsafe {
//!     // Pass to FFI as *const c_char
//!     let ptr = s.as_ptr();
//!     assert_eq!(CStr::from_ptr(ptr).to_str().unwrap(), "Pi = 3.14");
//! }
//! ```
//!
//! ## Design Notes
//!
//! - `CArrayString` uses a stack buffer for small strings and falls back to heap
//!   allocation for longer strings.
//! - The `cstr!` macro simplifies usage by automatically creating a `CArrayString<128>`
//!   with default stack size 128 bytes.
//! - The returned type is `CArrayString<128>`. The internal storage may be stack- or
//!   heap-based depending on the string length.
//!
//! ## Performance Considerations
//!
//! - Short strings that fit in the stack buffer avoid heap allocation.
//! - Long strings are automatically allocated on the heap.
//! - Choosing an appropriate buffer size (currently 128 bytes) can minimize heap fallback.
//!
//! ## Use Cases
//!
//! - Passing short strings to FFI calls without heap allocation.
//! - Performance-sensitive applications where allocation patterns matter.
//! - Embedded systems with constrained heap memory.
//!
//! ## Limitations
//!
//! - The default stack buffer size is fixed at 128 bytes in `cstr!`.
//! - Strings longer than 128 bytes will always be heap-allocated.
//!
//! ## See Also
//!
//! - [`CString`](std::ffi::CString) for explicit heap-allocated C strings
//! - [`CStr`](std::ffi::CStr) for borrowed C strings

pub mod c_array_string;
pub mod error;
pub mod macros;

pub use c_array_string::CArrayString;
pub use error::{CStrError, ContainsNulError};
