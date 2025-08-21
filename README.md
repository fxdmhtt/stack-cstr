# stack_cstr

`stack_cstr` is a high-performance Rust library for creating C-style strings (CStr/CString) efficiently.  
It tries to write the formatted string into a stack buffer first, and if the string is too long, it falls back to heap allocation.  
The final result is a safe C string that can be passed to FFI functions.

## Features

- Stack buffer attempt with configurable sizes
- Automatic heap fallback for long strings
- Supports `format_args!` style formatting
- Returns `Box<dyn CStrLike>` for easy FFI usage
- Simple macro interface: `cstr!()`
- Extensible stack sizes
