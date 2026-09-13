use crate::config::{HeaderField, ParamField, ProcessSortBy};
use persona_exporter_types::metrics::{
    ComponentListInfo, CpuListInfo, DiskInfo, MemoryInfo, NetworkInfo, ProcessListInfo,
    ServerMetrics, SystemInfo,
};
use surf::Client;

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
