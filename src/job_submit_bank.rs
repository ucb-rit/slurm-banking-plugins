extern crate reqwest;

// Based on: https://stackoverflow.com/a/33883281
#[repr(C)]
pub struct StaticCString(*const u8);
unsafe impl Sync for StaticCString {}

#[no_mangle]
pub static plugin_name: StaticCString =
    StaticCString(b"Slurm bank submit\0" as *const u8);

// Slurm
#[no_mangle]
pub extern fn job_submit() -> isize {
    println!("Job submitted");
    return 0;
}

#[no_mangle]
pub extern fn job_modify() -> isize {
    println!("Job modified");
    return 0;
}