#[macro_use]
extern crate lazy_static;

extern crate chrono;
extern crate config;
extern crate openapi;
extern crate rust_decimal;
extern crate slurm_banking;

use slurm_banking::accounting;
use slurm_banking::bindings::*;
use slurm_banking::logging;
use slurm_banking::safe_helpers;

use chrono::prelude::*;
use config::Config;
use std::collections::HashMap;
use std::os::raw::c_char;

static PLUGIN_NAME: &str = "jobcomp_bank";

lazy_static! {
    static ref SETTINGS: Config = {
        let mut conf = Config::default();
        match slurm_banking::prices_config::load_config_from_file(&mut conf) {
            Ok(_) => {}
            Err(_) => {}
        };
        conf
    };
}

// Static strings reference: https://stackoverflow.com/a/33883281
#[repr(C)]
pub struct StaticCString(*const u8);
unsafe impl Sync for StaticCString {}

// Begin static values required by Slurm
#[no_mangle]
pub static plugin_name: StaticCString = StaticCString(b"Slurm bank completion\0" as *const u8);

#[no_mangle]
pub static plugin_type: StaticCString = StaticCString(b"jobcomp/slurm_banking\0" as *const u8);

#[no_mangle]
pub static plugin_version: u32 = SLURM_VERSION_NUMBER;
// End public static values

fn log(message: &str) {
    logging::safe_info(&format!("{}: {}", PLUGIN_NAME, message));
}

fn log_with_jobid(jobid: &str, message: &str) {
    log(&format!("Job {}: {}", jobid, message));
}

// Slurm
#[no_mangle]
pub extern "C" fn init() -> u32 {
    SLURM_SUCCESS
}

#[no_mangle]
pub extern "C" fn fini() -> u32 {
    SLURM_SUCCESS
}

#[no_mangle]
pub extern "C" fn slurm_jobcomp_set_location(_location: *const c_char) -> u32 {
    SLURM_SUCCESS
}

#[no_mangle]
pub extern "C" fn slurm_jobcomp_log_record(job_ptr: *const job_record) -> u32 {
    log("slurm_jobcomp_log_record() invoked");
    // BEGIN: Check if this plugin should be enabled
    let conf = &SETTINGS;
    let plugin_enable_config = match conf.get::<HashMap<String, bool>>("Enable") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let enabled = plugin_enable_config
        .get("enable_job_complete_plugin")
        .unwrap_or(&false);
    if !enabled {
        return 0;
    }
    // END: Check if this plugin should be enabled

    let account: String = match safe_helpers::deref_cstr(unsafe { (*job_ptr).account }) {
        Some(account) => account,
        None => return ESLURM_INVALID_ACCOUNT,
    };
    let partition: String = match safe_helpers::deref_cstr(unsafe { (*job_ptr).partition }) {
        Some(partition) => partition,
        None => return ESLURM_INVALID_PARTITION_NAME,
    };
    let qos: String = match safe_helpers::deref_cstr(unsafe { (*(*job_ptr).qos_ptr).name }) {
        Some(qos) => qos,
        None => return ESLURM_INVALID_QOS,
    };
    let cpu_count = unsafe { (*job_ptr).total_cpus };
    let time_spent = (unsafe { (*job_ptr).end_time }) - (unsafe { (*job_ptr).start_time }); // in seconds

    let expected_cost = match accounting::expected_cost(&partition, cpu_count, time_spent, &conf) {
        Some(cost) => cost,
        None => return ESLURM_INTERNAL,
    };

    let jobslurmid = (unsafe { (*job_ptr).job_id }).to_string();
    let user_id = (unsafe { (*job_ptr).user_id }).to_string();

    let start_timestamp = unsafe { (*job_ptr).start_time };
    let start_timestamp_str =
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(start_timestamp, 0), Utc)
            .to_rfc3339();

    let end_timestamp = unsafe { (*job_ptr).end_time };
    let end_timestamp_str =
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(end_timestamp, 0), Utc)
            .to_rfc3339();

    let submit_timestamp = unsafe { (*(*job_ptr).details).submit_time };
    let submit_timestamp_str =
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(submit_timestamp, 0), Utc)
            .to_rfc3339();

    let nodes_raw = unsafe { (*job_ptr).nodes };
    let nodes = slurm_banking::range_format::expand_node_hostnames(
        &safe_helpers::deref_cstr(nodes_raw).unwrap_or("".to_string()),
    )
    .into_iter()
    .map(|name| openapi::models::Node::new(name))
    .collect();

    log_with_jobid(
        &jobslurmid,
        &format!(
            "Account: {:?}, Partition: {:?}, QoS: {:?}, CPUs: {:?}, Nodes: {:?}",
            account, partition, qos, cpu_count, nodes
        ),
    );

    let job_state = unsafe { (*job_ptr).job_state };
    let job_state_ptr = unsafe { job_state_string(job_state) };
    let job_state_str = safe_helpers::deref_cstr(job_state_ptr).unwrap();

    let num_req_nodes = unsafe { (*(*job_ptr).details).min_nodes };
    let num_alloc_nodes = unsafe { (*job_ptr).total_nodes };
    let raw_time_hr = (end_timestamp - start_timestamp) as f32 / 60.0 / 60.0;
    let cpu_time = cpu_count as f32 * raw_time_hr;

    let mut job_update_record = openapi::models::Job::new(jobslurmid.clone(), user_id, account);
    job_update_record.amount = Some(expected_cost.to_string());
    job_update_record.jobstatus = Some(job_state_str);
    job_update_record.partition = Some(partition);
    job_update_record.qos = Some(qos);
    job_update_record.submitdate = Some(submit_timestamp_str);
    job_update_record.startdate = Some(start_timestamp_str);
    job_update_record.enddate = Some(end_timestamp_str);
    job_update_record.nodes = Some(nodes);
    job_update_record.num_cpus = Some(cpu_count as i32);
    job_update_record.num_req_nodes = Some(num_req_nodes as i32);
    job_update_record.num_alloc_nodes = Some(num_alloc_nodes as i32);
    job_update_record.raw_time = Some(raw_time_hr);
    job_update_record.cpu_time = Some(cpu_time);

    log_with_jobid(
        &jobslurmid,
        &format!("Updating job with info: {:?}", job_update_record),
    );
    let base_path = slurm_banking::prices_config::get_base_path(&conf);
    let auth_token = slurm_banking::prices_config::get_auth_token(&conf);
    let _ = accounting::update_job(base_path, &auth_token, &jobslurmid, job_update_record);

    SLURM_SUCCESS
}

#[no_mangle]
pub extern "C" fn slurm_jobcomp_get_errno() -> u32 {
    return 3100;
}

#[no_mangle]
pub extern "C" fn slurm_jobcomp_strerror(_errnum: u32) -> *const c_char {
    std::ptr::null()
}

#[no_mangle]
pub extern "C" fn slurm_jobcomp_get_jobs(_job_cond: *const slurmdb_job_cond_t) -> List {
    let null: *const slurm_banking::bindings::xlist = std::ptr::null();
    return null as *mut slurm_banking::bindings::xlist;
}

#[no_mangle]
pub extern "C" fn slurm_jobcomp_archive(_arch_cond: *const slurmdb_archive_cond_t) -> u32 {
    return SLURM_SUCCESS;
}
