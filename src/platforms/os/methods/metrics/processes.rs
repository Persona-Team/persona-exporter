use std::cmp::Ordering;
use persona_exporter_types::metrics::ProcessInfo;
use sysinfo::{Pid, System};
use crate::config::ProcessSortBy;

pub fn get_process_by_id(sys: &System, pid: Pid, self_process: &mut ProcessInfo) {
    let system_process = sys.process(pid).unwrap();
    let process = ProcessInfo::from(system_process);

    self_process.status = process.status;
    self_process.disk_usage = process.disk_usage;
    self_process.cpu_usage = process.cpu_usage;
    self_process.memory_usage = process.memory_usage;
    self_process.virtual_memory = process.virtual_memory;
    self_process.run_time = process.run_time;
    self_process.start_time = process.start_time;

    self_process.name.push_str(&process.name);
    self_process.program_id.push_str(&process.program_id);
    self_process.user_id.push_str(&process.user_id);
    self_process.group_id.push_str(&process.group_id);
}

pub fn get_sort_closure(
    sort_by: &ProcessSortBy,
) -> impl Fn(&ProcessInfo, &ProcessInfo) -> Ordering {
    match sort_by {
        ProcessSortBy::Memory => {
            |a: &ProcessInfo, b: &ProcessInfo| b.memory_usage.cmp(&a.memory_usage)
        }
        ProcessSortBy::VirtualMemory => {
            |a: &ProcessInfo, b: &ProcessInfo| b.virtual_memory.cmp(&a.virtual_memory)
        }
        ProcessSortBy::RunTime => |a: &ProcessInfo, b: &ProcessInfo| b.run_time.cmp(&a.run_time),
        ProcessSortBy::StartTime => {
            |a: &ProcessInfo, b: &ProcessInfo| b.start_time.cmp(&a.start_time)
        }
        // default also contain ProcessSortBy::CpuUsage
        _ => |a: &ProcessInfo, b: &ProcessInfo| b.cpu_usage.total_cmp(&a.cpu_usage),
    }
}

pub fn collect_process_list_info(
    sys: &System,
    // process_list_buffer: &mut Vec<ProcessInfo>,
    process_list_buffer: &mut Vec<ProcessInfo>,
) {
    // let mut local_process_list_buffer = take(&mut process_list_struct.process_list);

    sys.processes().values().for_each(|process| {
        process_list_buffer.push(ProcessInfo::from(process));
    });

    // let self_process_metrics: Option<ProcessInfo> = get_current_pid()
    //     .ok()
    //     .and_then(|pid| sys.process(pid))
    //     .map(Into::into);

    // local_process_list_buffer.extend(
    //     sys.processes().values().map(|p| ProcessInfo::from_process(p))
    // );
    // sys
    //     .processes()
    //     .iter()
    //     .for_each(|process| process_list_buffer.push(ProcessInfo::from(process.1)));
    // .collect();

    // let sort_by_closure = get_sort_closure(sort_by);
    //
    // local_process_list_buffer.sort_unstable_by(sort_by_closure);
    // local_process_list_buffer.truncate(process_limit);
    // process_list_struct.process_list = local_process_list_buffer;

    // *process_list_struct = ProcessListInfo {
    //     exporter_metrics: self_process_metrics,
    //     process_list: local_process_list_buffer,
    // };
}
