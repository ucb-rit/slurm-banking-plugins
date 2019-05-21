use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn info(message: *const c_char);
}

/// Provides a safe interface for info logging
/// This usually shows up in `slurmctld.log`
pub fn safe_info(message: &str) {
    // Be wary of CString lifetimes with pointers
    // Reference: https://stackoverflow.com/a/52175101/8706910
    let message_cstring = CString::new(message)
        .expect("Could not create message");
    unsafe { info(message_cstring.as_ptr()); }
}