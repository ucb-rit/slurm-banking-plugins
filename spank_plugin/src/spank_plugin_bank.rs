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

use chrono::prelude::*;
use config::Config;
use std::collections::HashMap;
use std::os::raw::{c_char, c_int};
use std::sync::Mutex;

static PLUGIN_NAME: &str = "spank_bank";

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
pub static plugin_name: StaticCString = StaticCString(b"Slurm bank spank\0" as *const u8);

#[no_mangle]
pub static plugin_type: StaticCString = StaticCString(b"spank\0" as *const u8);

#[no_mangle]
pub static plugin_version: u32 = SLURM_VERSION_NUMBER;
// End public static values

fn log(message: &str) {
    logging::safe_spank_info(&format!("{}: {}", PLUGIN_NAME, message));
}

// Slurm
#[no_mangle]
pub extern "C" fn slurm_spank_init(sp: spank_t, _ac: c_int, _argv: *const *const c_char) -> c_int {
    let mut job_id: u32 = 0;
    let mut job_buffer_ptr: *mut job_info_msg_t = std::ptr::null_mut();
    unsafe {
        if spank_get_item(sp, spank_item_S_JOB_ID, &mut job_id as *mut u32) != 0 {
            return 0;
        }
        if slurm_load_job(&mut job_buffer_ptr as *mut *mut job_info_msg_t, job_id, SHOW_ALL as u16) != 0 {
            return 0;
        }
    }

    // BEGIN: Check if this plugin should be enabled
    let conf = SETTINGS.lock().unwrap();
    let plugin_enable_config = match conf.get::<HashMap<String, bool>>("Enable") {
        Ok(v) => v,
        Err(_) => return 0 
    };
    let enabled = plugin_enable_config.get("enable_spank_plugin").unwrap_or(&false);
    if !enabled {
        return 0
    }
    // END: Check if this plugin should be enabled

    let partition = safe_helpers::deref_cstr(unsafe { (*((*job_buffer_ptr).job_array)).partition }).unwrap();
    let qos = safe_helpers::deref_cstr(unsafe { (*((*job_buffer_ptr).job_array)).qos }).unwrap();
    let account = safe_helpers::deref_cstr(unsafe { (*((*job_buffer_ptr).job_array)).account }).unwrap();
    let max_cpus = unsafe { (*((*job_buffer_ptr).job_array)).max_cpus };
    let time_limit: i64 = unsafe { (*((*job_buffer_ptr).job_array)).time_limit } as i64;
    log(&format!("Partition: {:?}, QOS: {:?}, Account: {:?}, Max CPUs: {:?}, Time limit: {:?}", 
        partition, qos, account, max_cpus, time_limit));

    let expected_cost =
        match accounting::expected_cost(&partition, &qos, max_cpus, time_limit, &conf) {
            Some(cost) => cost,
            None => return 0,
        };

    let user_id = unsafe { (*((*job_buffer_ptr).job_array)).user_id };

    let start_timestamp = unsafe { (*((*job_buffer_ptr).job_array)).start_time };
    let start_timestamp_str = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(start_timestamp, 0), Utc).to_rfc3339();

    let job_create_record = swagger::models::Job::new(
        job_id.to_string(), user_id.to_string(), account, expected_cost.to_string())
        .with_jobstatus("RUNNING".to_string())
        .with_partition(partition)
        .with_qos(qos)
        .with_startdate(start_timestamp_str);

    log(&format!("Creating job wih info: {:?}", job_create_record));
    let base_path = slurm_banking::prices_config::get_base_path(&conf);
    let _ = accounting::create_job(base_path, job_create_record);

    unsafe { slurm_free_job_info_msg(job_buffer_ptr) };
    log(&format!("slurm_spank_init(). Job ID: {}", job_id));
    0
}