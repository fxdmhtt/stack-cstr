use std::ffi::CStr;

use stack_cstr::cstr;

#[test]
fn test_cstr_macro() {
    let c1 = cstr!("hi");

    let c2 = "x".repeat(100);
    let c2 = cstr!("{}", c2);

    let c3 = "x".repeat(300);
    let c3 = cstr!("{}", c3);

    assert_eq!(
        unsafe { CStr::from_ptr(c1.as_ptr()).to_str().unwrap() },
        "hi"
    );
    assert_eq!(
        unsafe { CStr::from_ptr(c2.as_ptr()).to_str().unwrap() },
        &"x".repeat(100)
    );
    assert_eq!(
        unsafe { CStr::from_ptr(c3.as_ptr()).to_str().unwrap() },
        &"x".repeat(300)
    );
}
