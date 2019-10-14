#[macro_use]
extern crate lazy_static;

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
use std::os::raw::{c_char, c_int};
use std::sync::Mutex;

static PLUGIN_NAME: &str = "spank_slurm_banking";

lazy_static! {
    static ref SETTINGS: Mutex<Config> = {
        let mut conf = Config::default();
        match slurm_banking::prices_config::load_config_from_file(&mut conf) {
            Ok(_) => {}
            Err(_) => {}
        };
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
pub extern "C" fn slurm_spank_init(sp: spank_t, _ac: c_int, _argv: *const *const c_char) -> u32 {
    let mut job_id: u32 = 0;
    let mut job_buffer_ptr: *mut job_info_msg_t = std::ptr::null_mut();
    unsafe {
        if spank_get_item(sp, spank_item_S_JOB_ID, &mut job_id as *mut u32) != 0 {
            return SLURM_SUCCESS;
        }
        if slurm_load_job(
            &mut job_buffer_ptr as *mut *mut job_info_msg_t,
            job_id,
            SHOW_ALL as u16,
        ) != 0
        {
            return SLURM_SUCCESS;
        }
    }

    // BEGIN: Check if this plugin should be enabled
    let conf = SETTINGS.lock().unwrap();
    let plugin_enable_config = match conf.get::<HashMap<String, bool>>("Enable") {
        Ok(v) => v,
        Err(_) => return SLURM_SUCCESS,
    };
    let enabled = plugin_enable_config
        .get("enable_spank_plugin")
        .unwrap_or(&false);
    if !enabled {
        return SLURM_SUCCESS;
    }
    // END: Check if this plugin should be enabled

    let partition =
        safe_helpers::deref_cstr(unsafe { (*((*job_buffer_ptr).job_array)).partition }).unwrap();
    let qos = safe_helpers::deref_cstr(unsafe { (*((*job_buffer_ptr).job_array)).qos }).unwrap();
    let account =
        safe_helpers::deref_cstr(unsafe { (*((*job_buffer_ptr).job_array)).account }).unwrap();
    let num_cpus = unsafe { (*((*job_buffer_ptr).job_array)).num_cpus };
    let max_cpus = unsafe { (*((*job_buffer_ptr).job_array)).max_cpus };
    let num_nodes = unsafe { (*((*job_buffer_ptr).job_array)).num_nodes };
    let max_nodes = unsafe { (*((*job_buffer_ptr).job_array)).max_nodes };
    let time_limit_minutes: i64 = unsafe { (*((*job_buffer_ptr).job_array)).time_limit } as i64; // in minutes
    let time_limit_seconds = time_limit_minutes * 60;
    log(&format!("Partition: {:?}, QOS: {:?}, Account: {:?}, Num CPUs: {:?}, Max CPUs: {:?}, Time limit: {:?}, Num nodes: {:?}, Max nodes: {:?}", 
        partition, qos, account, num_cpus, max_cpus, time_limit_seconds, num_nodes, max_nodes));

    let expected_cost =
        match accounting::expected_cost(&partition, &qos, num_cpus, time_limit_seconds, &conf) {
            Some(cost) => cost,
            None => return SLURM_SUCCESS,
        };

    let user_id = unsafe { (*((*job_buffer_ptr).job_array)).user_id };

    let start_timestamp = unsafe { (*((*job_buffer_ptr).job_array)).start_time };
    let start_timestamp_str =
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(start_timestamp, 0), Utc)
            .to_rfc3339();

    let submit_timestamp = unsafe { (*((*job_buffer_ptr).job_array)).start_time };
    let submit_timestamp_str =
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(submit_timestamp, 0), Utc)
            .to_rfc3339();

    let nodes_raw = unsafe { (*(*job_buffer_ptr).job_array).nodes };
    let nodes = slurm_banking::range_format::expand_node_hostnames(
        &safe_helpers::deref_cstr(nodes_raw).unwrap_or("".to_string()),
    )
    .into_iter()
    .map(|name| openapi::models::Node::new(name))
    .collect();
    log(&format!("Nodes: {:?}", nodes));

    let mut job_create_record =
        openapi::models::Job::new(job_id.to_string(), user_id.to_string(), account);
    job_create_record.amount = Some(expected_cost.to_string());
    job_create_record.jobstatus = Some("RUNNING".to_string());
    job_create_record.partition = Some(partition);
    job_create_record.qos = Some(qos);
    job_create_record.startdate = Some(start_timestamp_str);
    job_create_record.submitdate = Some(submit_timestamp_str);
    job_create_record.nodes = Some(nodes);
    job_create_record.num_cpus = Some(num_cpus as i32);
    job_create_record.num_alloc_nodes = Some(num_nodes as i32);

    log(&format!("Creating job wih info: {:?}", job_create_record));
    let base_path = slurm_banking::prices_config::get_base_path(&conf);
    let auth_token = slurm_banking::prices_config::get_auth_token(&conf);
    let _ = accounting::create_job(base_path, &auth_token, job_create_record);

    unsafe { slurm_free_job_info_msg(job_buffer_ptr) };
    log(&format!("slurm_spank_init(). Job ID: {}", job_id));
    SLURM_SUCCESS
}
