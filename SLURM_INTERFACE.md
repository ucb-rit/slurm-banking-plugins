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

## Notes
In a previous version, `info()` from Slurm was called incorrectly as a single-argument function instead of as a variable argument function. This did not cause errors in CentOS 7 testing and worked on BRC master node (SL6) until breaking (causing segfaults) in Slurm 20.

As it is now called with the correct matching function signature, this issue isn't expected to arise again. However, to mitigate the risk, other potential solutions include having the plugins log to their own file. This would add (probably unnecessary) complexity to the plugins, especially when we consider that multiple instances of the Spank plugin may be running at the same time on a given node. So the plugins would either have to implement their own locking control mechanism (to prevent race conditions from multiple processes writing to the same file) or log to a different file per job ID.
