# stack_cstr

`stack_cstr` is a high-performance Rust library for creating C-compatible strings (`&CStr`) efficiently.
It uses a stack buffer for short strings to avoid heap allocation, and automatically falls back to heap allocation for longer strings.
The resulting strings are safe to pass to FFI functions.

---

## Features

- Stack buffer allocation for short strings (default 128 bytes)
- Automatic heap fallback for longer strings
- Supports `format_args!` style formatting
- Returns `CArrayString<128>` for easy FFI usage
- Simple macro interface: `cstr!()`
- Ergonomic and safe for passing to C APIs

---

## Usage Example

```rust
use std::ffi::CStr;

use stack_cstr::cstr;

// Create a C-compatible string
let s = cstr!("Pi = {:.2}", 3.14159);
assert_eq!(s.as_c_str().to_str().unwrap(), "Pi = 3.14");

unsafe {
    // Pass to FFI as *const c_char
    let ptr = s.as_ptr();
    assert_eq!(CStr::from_ptr(ptr).to_str().unwrap(), "Pi = 3.14");
}
```