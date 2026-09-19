use crate::config::{AgentConfigFile, DataType};
// use crate::metrics::*;
use crate::platforms::os::methods::metrics::processes::*;

use crate::platforms::os::methods::arguments::{Buffers, CommonMetricsInformation, RequestBodyOptions};
use crate::platforms::os::methods::{
    build_request_body, collect_metrics_as_line_protocol, create_metrics_struct_by_config,
    get_host, send_request,
};
use persona_exporter_types::metrics::ProcessInfo;
use persona_exporter_types::metrics::traits::Clear;
use std::time::{Duration, SystemTime};
use surf::{Client, RequestBuilder};
use sysinfo::{Components, Disks, Networks, Process, ProcessesToUpdate, System, get_current_pid};
use tracing::info;
use url::Url;
use crate::platforms::os::methods::metrics::components::collect_components_metrics;
use crate::platforms::os::methods::metrics::cpu::collect_cpus_metrics;
use crate::platforms::os::methods::metrics::disk::collect_disk_metrics;
use crate::platforms::os::methods::metrics::memory::collect_memory_metrics;
use crate::platforms::os::methods::metrics::network::collect_network_metrics;
use crate::platforms::os::methods::metrics::system::collect_system_metrics;

pub async fn collect_metrics_for_os(config: AgentConfigFile) {
    let mut sys = (config.metrics.cpu.settings.enabled
        || config.metrics.memory.settings.enabled
        || config.metrics.processes.settings.enabled
        || config.metrics.system.settings.enabled)
        .then(sysinfo::System::new);
    let mut disks = config
        .metrics
        .disks
        .settings
        .enabled
        .then(Disks::new_with_refreshed_list);
    let mut networks = config
        .metrics
        .network
        .settings
        .enabled
        .then(Networks::new_with_refreshed_list);
    let mut components = config
        .metrics
        .components
        .settings
        .enabled
        .then(Components::new_with_refreshed_list);

    // let common_metrics = CommonMetricsInformation {
    //     components: components,
    //     networks: networks,
    //     disks: disks,
    //     system: sys,
    // };





    info!("Exporter initialized");
    let additional_headers = &config.server.push.http_headers;
    let get_params = &config.server.push.url_params;
    let target_url = &config.server.push.url;
    let await_sec = config.agent.send_interval;

    let client: Client = surf::Config::new()
        .set_base_url(Url::parse(target_url).unwrap())
        .try_into()
        .unwrap();

    let request_options = RequestBodyOptions {
        client,
        url: target_url.clone(),
        host: get_host(target_url),
        get_params: get_params.clone(),
        headers: additional_headers.clone(),
    };

    info!("Starting persona-exporter");


    // let mut metrics: ServerMetrics;
    let mut line_protocol_buffer: Vec<u8> = Vec::new();
    // let mut line_protocol_options: ToLineProtocolOptions;

    let mut buffers = Buffers {
        metrics: create_metrics_struct_by_config(&config),
        ..Buffers::default()
    };
    // let mut process_list_options = CollectProcessListOptions {
    //     sort_by: config.metrics.processes.sort_by,
    //     cpu_cores: cpu_core_count,
    //     process_limit: config.metrics.processes.process_limit,
    // };
    let physical_core_count = System::physical_core_count().unwrap_or(0);
    let sort_by = get_sort_closure(&config.metrics.processes.sort_by);
    let process_limit = config.metrics.processes.process_limit;
    // let mut time: i64;

    loop {
        info!("Collect metrics...");

        // Метрики требующий sysinfo::System
        if let Some(ref mut s) = sys {
            if let Some(ref mut mem_buf) = buffers.metrics.memory && config.metrics.memory.settings.enabled {
                s.refresh_memory();
                // mem_buf.clear_dynamic();
                collect_memory_metrics(s, mem_buf);
            }
            if let Some(ref mut cpu_buf) = buffers.metrics.cpu && config.metrics.cpu.settings.enabled {
                s.refresh_cpu_all();
                cpu_buf.clear_dynamic();
                collect_cpus_metrics(s, physical_core_count, cpu_buf);
            }
            if let Some(ref mut system_buf) = buffers.metrics.system && config.metrics.system.settings.enabled {
                system_buf.clear_dynamic();
                collect_system_metrics(system_buf);
            }
            if let Some(ref mut process_list_buf) = buffers.metrics.process_list && config.metrics.processes.settings.enabled{
                s.refresh_processes(
                    ProcessesToUpdate::All,
                    config.metrics.processes.remove_dead_processes,
                );
                process_list_buf.clear_dynamic();

                // Информация о процессах
                collect_process_list_info(s, &mut process_list_buf.process_list);
                //// Сортируем по заданной функции
                process_list_buf.process_list.sort_unstable_by(&sort_by);
                //// Обрезаем готовый массив
                process_list_buf.process_list.truncate(process_limit);

                // Отдельная информация о самом экспортере
                let self_process: Option<ProcessInfo> = get_current_pid()
                    .ok()
                    .and_then(|pid| s.process(pid))
                    .map(|process: &Process| ProcessInfo::from(process));

                process_list_buf.exporter_metrics = self_process;
            }
        }

        // if let Some(ref mut disk_buffer) = buffers.metrics.disk {
        //     disks.as_mut().map(|d| {
        //         d.refresh(false);
        //         collect_disk_metrics(d, "/", disk_buffer);
        //     });
        // }
        if let (Some(disk_buffer), Some(d)) = (&mut buffers.metrics.disk, &mut disks) {
            d.refresh(false);
            disk_buffer.clear_dynamic();
            collect_disk_metrics(d, "/", disk_buffer);
        }
        if let (Some(network_buffer), Some(n)) = (&mut buffers.metrics.network, &mut networks) {
            n.refresh(false);
            network_buffer.clear_dynamic();
            collect_network_metrics(n, network_buffer);
        }
        if let (Some(components_info), Some(c)) = (&mut buffers.metrics.components, &mut components)
        {
            c.refresh(false);
            components_info.clear_dynamic();
            collect_components_metrics(c, components_info);
        }

        buffers.metrics.time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as i64;

        let mut request: RequestBuilder = build_request_body(&request_options);

        match config.agent.data_type {
            DataType::LineProtocol => {
                line_protocol_buffer.clear();
                collect_metrics_as_line_protocol(&buffers.metrics, &mut line_protocol_buffer);

                // line_protocol_buffer = String::from_utf8(collect_metrics_as_line_protocol(&line_protocol_options).to_vec()).unwrap_or_default();
                info!("Sending data to a specified URL",);

                request = request
                    // .header(http::header::CONTENT_LENGTH.as_str(), &line_protocol_buffer.len().to_string())
                    .body_bytes(&line_protocol_buffer);
            }
            DataType::Json => {
                info!("Machine metrics: {:#?}", buffers.metrics);

                // let json_metrics = serde_json::to_string(&machine_metrics).expect("Failed to serialize to json");
                let json_body = serde_json::json!(buffers.metrics);

                request = request
                    .body_json(&json_body)
                    .expect("Failed to create request body");
            }
        }

        send_request(request, &request_options.client).await;

        info!("Next metrics before {} seconds", await_sec);
        smol::Timer::after(Duration::from_secs(await_sec)).await;
    }
}
