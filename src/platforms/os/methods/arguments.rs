use crate::config::{HeaderField, ParamField, ProcessSortBy};
use persona_exporter_types::metrics::{
    ComponentListInfo, CpuListInfo, DiskInfo, MemoryInfo, NetworkInfo, ProcessListInfo,
    ServerMetrics, SystemInfo,
};
use surf::Client;
use sysinfo::{Components, Disks, Networks};

#[derive(Default)]
pub struct ToLineProtocolOptions {
    // pub time: i64,
    pub system: SystemInfo,
    pub memory: MemoryInfo,
    pub disk: DiskInfo,
    pub network: NetworkInfo,
    pub cpu: CpuListInfo,
    pub components: ComponentListInfo,
    pub processes_info: ProcessListInfo,
}

pub struct RequestBodyOptions {
    pub client: Client,
    pub url: String,
    pub host: String,
    pub get_params: Vec<ParamField>,
    pub headers: Vec<HeaderField>,
}

pub struct CollectProcessListOptions {
    pub sort_by: ProcessSortBy,
    pub cpu_cores: usize,
    pub process_limit: usize,
}

#[derive(Default)]
pub struct LineProtocolBuffer {
    pub line_protocol_buffer: Vec<u8>,
    pub line_protocol_options: ToLineProtocolOptions,
}

#[derive(Default)]
pub struct Buffers {
    pub metrics: ServerMetrics,
    pub line_protocol: LineProtocolBuffer,
}

pub struct CommonMetricsInformation {
    pub components: Option<sysinfo::Components>,
    pub networks: Option<sysinfo::Networks>,
    pub disks: Option<sysinfo::Disks>,
    pub system: Option<sysinfo::System>,
}

pub struct CommonConfigurations {
    pub memory_is_enabled: bool,
    pub system_is_enabled: bool,
    pub components_is_enabled: bool,
    pub cpu_is_enabled: bool,
    pub processes_is_enabled: bool,
    pub networks_is_enabled: bool,
    pub disks_is_enabled: bool,

    pub cpu_physical_core_count: usize,
    pub process_list_limit: usize,
}

