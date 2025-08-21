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
