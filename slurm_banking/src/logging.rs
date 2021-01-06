use std::ffi::CString;
// use std::os::raw::c_char;

use super::bindings::*;

extern crate openapi;

// extern "C" {
//    fn info(message: *const c_char);
// }

/// Provides a safe interface for info logging
/// This usually shows up in `slurmctld.log`
pub fn safe_info(message: &str) {
    // Be wary of CString lifetimes with pointers
    // i.e., do not refactor as `info(CString::new(message).unwrap().as_ptr())`
    // Reference: https://stackoverflow.com/a/52175101/8706910
    let format_cstring = CString::new("%s").expect("Could not create format string");
    let message_cstring = CString::new(message).expect("Could not create message");
    unsafe {
        slurm_info(format_cstring.as_ptr(), message_cstring.as_ptr());
    }
}

pub fn safe_spank_info(message: &str) {
    let format_cstring = CString::new("%s").expect("Could not create format string");
    let message_cstring = CString::new(message).expect("Could not create message");
    unsafe {
        slurm_info(format_cstring.as_ptr(), message_cstring.as_ptr());
    }
}

pub fn safe_spank_error(message: &str) {
    let message_cstring = CString::new(message).expect("Could not create message");
    unsafe {
        slurm_error(message_cstring.as_ptr());
    }
}

fn display_option<T: std::fmt::Debug>(option: &Option<T>) -> String {
    match option {
        Some(x) => format!("{:?}", x),
        None => "None".to_string(),
    }
}

pub fn display_job_record(job_record: &openapi::models::Job) -> String {
    format!(
        "jobslurmid: {jobslurmid}, \
         submitdate: {submitdate}, \
         startdate: {startdate}, \
         enddate: {enddate}, \
         userid: {userid}, \
         accountid: {accountid}, \
         amount: {amount}, \
         jobstatus: {jobstatus}, \
         partition: {partition}, \
         qos: {qos}, \
         nodes: {nodes}, \
         num_cpus: {num_cpus}, \
         num_req_nodes: {num_req_nodes}, \
         num_alloc_nodes: {num_alloc_nodes}, \
         raw_time: {raw_time}, \
         cpu_time: {cpu_time}",
        jobslurmid = job_record.jobslurmid,
        submitdate = display_option(&job_record.submitdate),
        startdate = display_option(&job_record.startdate),
        enddate = display_option(&job_record.enddate),
        userid = job_record.userid,
        accountid = job_record.accountid,
        amount = display_option(&job_record.amount),
        jobstatus = display_option(&job_record.jobstatus),
        partition = display_option(&job_record.partition),
        qos = display_option(&job_record.qos),
        nodes = display_option(&job_record.nodes),
        num_cpus = display_option(&job_record.num_cpus),
        num_req_nodes = display_option(&job_record.num_req_nodes),
        num_alloc_nodes = display_option(&job_record.num_alloc_nodes),
        raw_time = display_option(&job_record.raw_time),
        cpu_time = display_option(&job_record.cpu_time)
    )
}
