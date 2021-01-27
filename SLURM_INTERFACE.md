# Slurm Interface
This documents the Slurm API/functions used by each of the plugins.

## Plugin library (`slurm_banking`)
- `pub fn slurm_info(format: *const ::std::os::raw::c_char, ...);`

## `job_submit_plugin`
- `pub const SLURM_VERSION_NUMBER: u32`
- `pub const SLURM_SUCCESS: u32`
- `pub struct job_descriptor`
  - `pub partition: *mut ::std::os::raw::c_char`
  - `pub user_id: u32`
  - `pub account: *mut ::std::os::raw::c_char`
  - `pub cpus_per_task: u16`
  - `pub num_tasks: u32`
  - `pub time_limit: u32`
- `*error_msg = xstrdup(msg.as_ptr())`

## `spank_plugin`
- `pub const SLURM_VERSION_NUMBER: u32`
- `pub const SLURM_SUCCESS: u32`
- `pub fn spank_get_item(spank: spank_t, item: spank_item_t, ...) -> spank_err_t`
- `pub fn slurm_load_job(resp: *mut *mut job_info_msg_t, job_id: u32, show_flags: u16) -> ::std::os::raw::c_int;`
- `pub type job_info_msg_t = job_info_msg;`
  - `pub job_array: *mut slurm_job_info_t`
    - `pub partition: *mut ::std::os::raw::c_char`
    - `pub qos: *mut ::std::os::raw::c_char`
    - `pub account: *mut ::std::os::raw::c_char`
    - `pub num_cpus: u32`
    - `pub max_cpus: u32`
    - `pub num_nodes: u32`
    - `pub max_nodes: u32`
    - `pub time_limit: u32`
    - `pub user_id: u32`
    - `pub start_time: time_t`
    - `pub submit_time: time_t`
    - `pub nodes: *mut ::std::os::raw::c_char`
- `pub fn slurm_free_job_info_msg(job_buffer_ptr: *mut job_info_msg_t);`

## `job_completion_plugin`
- `pub const SLURM_VERSION_NUMBER: u32`
- `pub const SLURM_SUCCESS: u32`
- `pub struct job_record`
  - `pub account: *mut ::std::os::raw::c_char`
  - `pub part_ptr: *mut part_record`
    - `pub name: *mut ::std::os::raw::c_char`
  - `pub qos_ptr: *mut slurmdb_qos_rec_t`
    - `pub name: *mut ::std::os::raw::c_char`
  - `pub total_cpus: u32`
  - `pub end_time: time_t`
  - `pub start_time: time_t`
  - `pub job_id: u32`
  - `pub user_id: u32`
  - `pub details: *mut job_details`
    - `pub submit_time: time_t`
    - `pub min_nodes: u32`
  - `pub nodes: *mut ::std::os::raw::c_char`
  - `pub job_state: u32`
  - `pub total_nodes: u32`
- `pub fn job_state_string(inx: u32) -> *mut ::std::os::raw::c_char;`
