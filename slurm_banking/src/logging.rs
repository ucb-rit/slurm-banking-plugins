use std::ffi::CString;
use std::os::raw::c_char;

use super::bindings::*;

extern "C" {
    fn info(message: *const c_char);
}

/// Provides a safe interface for info logging
/// This usually shows up in `slurmctld.log`
pub fn safe_info(message: &str) {
    // Be wary of CString lifetimes with pointers
    // i.e., do not refactor as `info(CString::new(message).unwrap().as_ptr())`
    // Reference: https://stackoverflow.com/a/52175101/8706910
    let message_cstring = CString::new(message).expect("Could not create message");
    unsafe {
        info(message_cstring.as_ptr());
    }
}

pub fn safe_spank_info(message: &str) {
    let message_cstring = CString::new(message).expect("Could not create message");
    unsafe {
        slurm_info(message_cstring.as_ptr());
    }
}

pub fn safe_spank_error(message: &str) {
    let message_cstring = CString::new(message).expect("Could not create message");
    unsafe {
        slurm_error(message_cstring.as_ptr());
    }
}