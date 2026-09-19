use std::path::Path;
use persona_exporter_types::DEFAULT_UNKNOWN_MESSAGE;
use persona_exporter_types::metrics::DiskInfo;
use sysinfo::Disks;

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