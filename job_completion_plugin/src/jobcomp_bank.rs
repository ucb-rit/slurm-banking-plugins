#[macro_use]
extern crate lazy_static;

extern crate config;
extern crate rust_decimal;
extern crate slurm_banking;
extern crate swagger;

use slurm_banking::accounting;
use slurm_banking::bindings::*;
use slurm_banking::logging;
use slurm_banking::safe_helpers;

use config::Config;
use std::collections::HashMap;
use std::os::raw::c_char;
use std::sync::Mutex;

static PRICES_CONFIG_FILE_PATH: &str = "/etc/slurm/prices";
static PLUGIN_NAME: &str = "jobcomp_bank";

lazy_static! {
    static ref SETTINGS: Mutex<Config> = {
        let mut conf = Config::default();
        slurm_banking::prices_config::load_config_from_file(&mut conf).unwrap();
        Mutex::new(conf)
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
pub static plugin_type: StaticCString = StaticCString(b"jobcomp/bank\0" as *const u8);

#[no_mangle]
pub static plugin_version: u32 = SLURM_VERSION_NUMBER;
// End public static values

fn log(message: &str) {
    logging::safe_info(&format!("{}: {}", PLUGIN_NAME, message));
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
    let account: String = match safe_helpers::deref_cstr(unsafe { (*job_ptr).account }) {
        Some(account) => account,
        None => return ESLURM_INVALID_ACCOUNT,
    };
    let job_id = unsafe { (*job_ptr).job_id };
    let partition: String = match safe_helpers::deref_cstr(unsafe { (*job_ptr).partition }) {
        Some(partition) => partition,
        None => return ESLURM_INVALID_PARTITION_NAME,
    };
    let qos: String = (unsafe { (*job_ptr).qos_id }).to_string(); // TODO: change to qos_ptr
    let cpu_count = unsafe { (*job_ptr).cpu_cnt };
    let time_spent = ((unsafe { (*job_ptr).end_time }) - (unsafe { (*job_ptr).start_time })) / 60;

    log(&format!("account: {:?}, job id: {:?}, cpu_count: {:?}, time_spent: {:?}", 
        account, job_id, cpu_count, time_spent));

    let conf = SETTINGS.lock().unwrap();
    let expected_cost =
        match accounting::expected_cost(&partition, &qos, cpu_count, time_spent, &conf) {
            Some(cost) => cost,
            None => return ESLURM_INTERNAL,
        };

    let jobslurmid = (unsafe { (*job_ptr).job_id }).to_string();
    let user_id = (unsafe { (*job_ptr).user_id}).to_string();
    // let job_state = (unsafe { (*job_ptr).job_state });
    // let job_state_ptr = unsafe { job_state_string(job_state) };
    // let job_state_str = safe_helpers::deref_cstr(job_state_ptr).unwrap();

    let job_update_record = swagger::models::Job::new(
        jobslurmid.clone(), user_id, account, expected_cost.to_string())
        .with_jobstatus("COMPLETING".to_string())
        .with_partition(partition)
        .with_qos(qos);

    accounting::update_job(&jobslurmid, job_update_record);

    // unsafe { free(job_state_ptr as *mut std::ffi::c_void); }

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
