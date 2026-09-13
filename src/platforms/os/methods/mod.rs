pub mod arguments;

use crate::config::AgentConfigFile;
use crate::platforms::os::methods::arguments::RequestBodyOptions;
use influxdb_line_protocol::LineProtocolBuilder;
use persona_exporter_types::metrics::{
    ComponentListInfo, CpuListInfo, DiskInfo, MemoryInfo, NetworkInfo,
    ProcessListInfo, ServerMetrics, SystemInfo,
};
// use persona_exporter_types::traits::line_protocol::{FromWithMeasurement, IntoWithMeasurement};
use persona_exporter_types::traits::line_protocol::{
    FinishLineProtocol, FromWithMeasurement,
};
use std::collections::BTreeMap;
use surf::post;
use tracing::{debug, error, info};
use url::Url;

pub fn collect_metrics_as_line_protocol(
    metrics: &ServerMetrics,
    line_buffer: &mut Vec<u8>,
) {
    let time = metrics.time;
    if let Some(ref system) = metrics.system {
        line_buffer.extend_from_slice(
            LineProtocolBuilder::from_with_name(system, "metrics_system")
                .finish(time)
                .as_slice(),
        );
    }
    if let Some(ref disk) = metrics.disk {
        line_buffer.extend_from_slice(
            LineProtocolBuilder::from_with_name(disk, "metrics_disk")
                .finish(time)
                .as_slice(),
        );
    }
    if let Some(ref network) = metrics.network {
        line_buffer.extend_from_slice(
            LineProtocolBuilder::from_with_name(network, "metrics_network")
                .finish(time)
                .as_slice(),
        );
    }
    if let Some(ref cpu) = metrics.cpu {
        line_buffer.extend_from_slice(
            LineProtocolBuilder::from_with_name(cpu, "metrics_cpu")
                .finish(time)
                .as_slice(),
        );
        cpu.cpu_cores.iter().for_each(
            |cpu_core| {
                line_buffer.extend_from_slice(LineProtocolBuilder::from_with_name(cpu_core, "mertics_cpu_cores")
                    .finish(time)
                    .as_slice()
                );
            }
        );
    }
    if let Some(ref memory) = metrics.memory {
        line_buffer.extend_from_slice(
            LineProtocolBuilder::from_with_name(memory, "metrics_memory")
                .finish(time)
                .as_slice(),
        );
    }

    if let Some(ref component_list) = metrics.components {
        component_list.components.iter().for_each(|c| {
            line_buffer.extend_from_slice(
                LineProtocolBuilder::from_with_name(c, "metrics_component_list")
                    .finish(time)
                    .as_slice(),
            );
        });
    }
    if let Some(ref processes) = metrics.process_list {
        processes.process_list.iter().for_each(|p| {
            line_buffer.extend_from_slice(
                LineProtocolBuilder::from_with_name(p, "metrics_process_list")
                    .finish(time)
                    .as_slice(),
            );
        });
        processes.exporter_metrics.iter().for_each(|self_process| {
            line_buffer.extend_from_slice(
                LineProtocolBuilder::from_with_name(self_process, "metrics_process_list")
                    .finish(time)
                    .as_slice(),
            );
        });
    }
}

pub fn build_request_body(options: &RequestBodyOptions) -> surf::RequestBuilder {
    let total_url = options.url.clone();

    // if !options.get_params.is_empty() {
    //     let mut query_string = String::new();
    //     let get_url_pairs = form_urlencoded::Serializer::new(&mut query_string);
    //
    //     options
    //         .get_params
    //         .iter()
    //         .fold(get_url_pairs, |mut acc, get_param| {
    //             acc.append_pair(get_param.key.as_str(), get_param.value.as_str());
    //             acc
    //         })
    //         .finish();
    //     total_url = format!("{}?{}", total_url, query_string);
    // }
    // let headers: Vec<(&str, &str)> = options
    //     .headers
    //     .iter()
    //     .map(|h| (h.key.as_str(), h.value.as_str()))
    //     .collect();

    let mut query_params: BTreeMap<String, String> = BTreeMap::new();
    for params in &options.get_params {
        query_params.insert(params.key.clone(), params.value.clone());
    }
    let mut request = post(total_url)
        .query(&query_params)
        .unwrap()
        .header(http::header::HOST.as_str(), &options.host)
        .header(http::header::CONNECTION.as_str(), "close");

    info!("{:#?}", request);
    for header in &options.headers {
        request = request.header(header.key.as_str(), header.value.as_str());
    }

    request
}

pub async fn send_request(request: surf::RequestBuilder, _client: &surf::Client) {
    let response = request.send().await;

    match response {
        Ok(mut success_response) => {
            let response_status = success_response.status();
            debug!("{:#?}", success_response);
            info!(
                "{}",
                success_response.body_string().await.unwrap_or_default()
            );
            info!(
                "Response status: {} \"{}\"",
                response_status as u16,
                response_status.canonical_reason()
            );
        }
        Err(err) => {
            error!("Send error: {}", err)
        }
    }
}

pub fn load_config() -> AgentConfigFile {
    AgentConfigFile::new().unwrap_or_else(|err| {
        error!("Something is wrong in your config file");
        panic!("{}", err);
    })
}

pub fn initial_tracing(debug: bool) {
    tracing_subscriber::fmt()
        .compact()
        .without_time()
        .with_target(false)
        .with_env_filter(if debug { "debug" } else { "info" })
        .init();
}

pub fn get_host(url: &str) -> String {
    if let Ok(parsed_url) = Url::parse(url) {
        return parsed_url.host_str().unwrap_or("localhost").to_string();
    };
    "incorrect_url".to_string()
}

pub fn create_metrics_struct_by_config(config: &AgentConfigFile) -> ServerMetrics {
    ServerMetrics {
        system: config
            .metrics
            .system
            .settings
            .enabled
            .then(SystemInfo::default),
        process_list: config
            .metrics
            .processes
            .settings
            .enabled
            .then(ProcessListInfo::default),
        memory: config
            .metrics
            .memory
            .settings
            .enabled
            .then(MemoryInfo::default),
        disk: config
            .metrics
            .disks
            .settings
            .enabled
            .then(DiskInfo::default),
        network: config
            .metrics
            .network
            .settings
            .enabled
            .then(NetworkInfo::default),
        cpu: config
            .metrics
            .cpu
            .settings
            .enabled
            .then(CpuListInfo::default),
        components: config
            .metrics
            .components
            .settings
            .enabled
            .then(ComponentListInfo::default),
        time: 0,
    }
}

// pub fn parse_cli_arguments()
