use crate::config::ProcessSortBy;
use persona_exporter_types::DEFAULT_UNKNOWN_MESSAGE;
use persona_exporter_types::metrics::*;
use std::cmp::Ordering;
use std::path::Path;
use sysinfo::{Components, Disks, Networks, Pid, System};

pub fn collect_cpus_metrics(
    sys: &System,
    physical_core_count: usize,
    cpu_list_buffer: &mut CpuListInfo,
) {
    cpu_list_buffer.cpu_usage = sys.global_cpu_usage();
    cpu_list_buffer.threads = sys.cpus().len();
    cpu_list_buffer.physical_core_count = physical_core_count;

    sys.cpus().iter().for_each(|c| {
        cpu_list_buffer.cpu_cores.push(CpuCoreInfo {
            frequency: c.frequency(),
            os_name: c.name().to_string(),
            cpu_usage: c.cpu_usage(),
        });
    });
}

pub fn collect_memory_metrics(sys: &System, memory_buffer: &mut MemoryInfo) {
    memory_buffer.total_memory = sys.total_memory();
    memory_buffer.used_memory = sys.used_memory();
    memory_buffer.free_memory = sys.free_memory();
    memory_buffer.available_memory = sys.available_memory();
    memory_buffer.total_swap = sys.total_swap();
    memory_buffer.used_swap = sys.used_swap();
    memory_buffer.free_swap = sys.free_swap();
}

pub fn collect_disk_metrics(disks: &mut Disks, mount_point: &str, disk_buffer: &mut DiskInfo) {
    let disk = disks
        .iter()
        .find(|disk| disk.mount_point() == Path::new(&mount_point));

    if let Some(disk) = disk {
        disk_buffer.name.push_str(&disk.name().to_string_lossy());
        disk_buffer
            .file_system
            .push_str(&disk.file_system().to_string_lossy());
        disk_buffer.kind.push_str(&disk.kind().to_string());
        disk_buffer.total_space = disk.total_space();
        disk_buffer.available_space = disk.available_space();
    } else {
        disk_buffer.name.push_str(DEFAULT_UNKNOWN_MESSAGE);
        disk_buffer.file_system.push_str(DEFAULT_UNKNOWN_MESSAGE);
        disk_buffer.kind.push_str(DEFAULT_UNKNOWN_MESSAGE);
    }
}

pub fn collect_network_metrics(networks: &mut Networks, network_info_buffer: &mut NetworkInfo) {
    let main_interface = networks
        .iter()
        .filter(|(name, _)| {
            let n = name.as_str();
            n != "lo" && !n.starts_with("br-") && !n.starts_with("docker") && !n.starts_with("veth")
        })
        .max_by_key(|(_, data)| data.total_received() + data.total_transmitted())
        .map(|(name, _)| name.clone());

    if let Some(interface) = main_interface
        && let Some(data) = networks.get(&interface)
    {
        network_info_buffer.interface_name.push_str(&interface);
        network_info_buffer.total_rx_bytes = data.total_received();
        network_info_buffer.total_tx_bytes = data.total_transmitted();
        network_info_buffer.total_rx_packets = data.total_packets_transmitted();
        network_info_buffer.total_tx_packets = data.total_packets_transmitted();
        network_info_buffer.total_rx_errors = data.total_errors_on_received();
        network_info_buffer.total_tx_errors = data.total_errors_on_transmitted();
    } else {
        network_info_buffer
            .interface_name
            .push_str(DEFAULT_UNKNOWN_MESSAGE);
    }
}

pub fn collect_components_metrics(
    components: &mut Components,
    components_list_buffer: &mut ComponentListInfo,
) {
    components_list_buffer.count = components.len();
    components_list_buffer.is_empty = components.is_empty();

    components.iter().for_each(|c| {
        components_list_buffer.components.push(ComponentInfo {
            id: c.id().unwrap_or(DEFAULT_UNKNOWN_MESSAGE).to_string(),
            name: c.label().to_string(),
            temp: c.temperature().unwrap_or(0.0),
            critical_temp: c.critical().unwrap_or(0.0),
            max_temp: c.max().unwrap_or(0.0),
        });
    });
}

pub fn collect_system_metrics(system_metrics_buffer: &mut SystemInfo) {
    system_metrics_buffer
        .name
        .push_str(System::name().as_deref().unwrap_or(DEFAULT_UNKNOWN_MESSAGE));
    system_metrics_buffer.kernel_version.push_str(
        System::kernel_version()
            .as_deref()
            .unwrap_or(DEFAULT_UNKNOWN_MESSAGE),
    );
    system_metrics_buffer.os_version.push_str(
        System::os_version()
            .as_deref()
            .unwrap_or(DEFAULT_UNKNOWN_MESSAGE),
    );
    system_metrics_buffer.host_name.push_str(
        System::host_name()
            .as_deref()
            .unwrap_or(DEFAULT_UNKNOWN_MESSAGE),
    );

    system_metrics_buffer
        .kernel_long_version
        .push_str(&System::kernel_long_version());
    system_metrics_buffer.cpu_arch.push_str(&System::cpu_arch());
    system_metrics_buffer
        .distribution_id
        .push_str(&System::distribution_id());
    system_metrics_buffer
        .distribution_id_like
        .push(System::distribution_id_like().join(","));
    system_metrics_buffer.boot_time = System::boot_time();
    system_metrics_buffer.uptime = System::uptime();
    system_metrics_buffer.load_average = LoadAverage::from(System::load_average());

    // *system_metrics_buffer = SystemInfo {
    //     name: System::name().unwrap_or(DEFAULT_UNKNOWN_MESSAGE.to_string()),
    //     kernel_long_version: System::kernel_long_version(),
    //     kernel_version: System::kernel_version().unwrap_or(DEFAULT_UNKNOWN_MESSAGE.to_string()),
    //     distribution_id: System::distribution_id(),
    //     distribution_id_like: System::distribution_id_like(),
    //     cpu_arch: System::cpu_arch(),
    //     boot_time: System::boot_time(),
    //     uptime: System::uptime(),
    //     os_version: System::os_version().unwrap_or(DEFAULT_UNKNOWN_MESSAGE.to_string()),
    //     host_name: System::host_name().unwrap_or(DEFAULT_UNKNOWN_MESSAGE.to_string()),
    //     load_average: load_avg,
    // };
}

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
