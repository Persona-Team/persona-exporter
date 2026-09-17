use crate::config::*;
use config::{Config, ConfigError};
use config_shellexpand::TemplatedFile;
use std::env;
use std::fs::{create_dir_all, write};
use std::path::PathBuf;
use tracing::{info, warn};

const CONFIG_FILENAME: &str = "config.yaml";

impl AgentConfigFile {
    pub fn new() -> Result<Self, ConfigError> {
        let config_path: PathBuf = {
            env::var("PERSONA_EXPORTER_CONFIG_PATH")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    if cfg!(target_os = "linux") {
                        PathBuf::from("/etc/persona-exporter")
                    } else {
                        // For Windows
                        PathBuf::from(
                            env::var("ProgramData")
                                .unwrap_or_else(|_| r"C:\Program Data".to_string()),
                        )
                        .join("PersonaMetrics")
                        .join("PersonaExporter")
                    }
                    .join(CONFIG_FILENAME)
                })
        };

        if !config_path.exists() {
            create_dir_all(&config_path)
                .expect("Something went wrong. Failed to create directories");

            let write_result = write(&config_path, include_str!("../config.example.yaml"));

            match write_result {
                Ok(_) => {
                    info!("Successfully insert default config to {:?}", config_path);
                }
                Err(err) => {
                    warn!(
                        "Failed to insert default config to {:?}. File has been created, but still empty - {}",
                        config_path, err
                    );
                }
            }
        }

        info!("You might change config directory through env var 'PERSONA_EXPORTER_CONFIG_PATH'");
        info!("Current full config path: {:?}", config_path);

        Config::builder()
            .add_source(Config::try_from(&Self::default())?)
            .add_source(TemplatedFile::with_name(config_path).required(false))
            .add_source(config::Environment::with_prefix("PE").separator("__"))
            .build()?
            .try_deserialize()
    }
}

impl Default for AgentConfigFile {
    fn default() -> Self {
        AgentConfigFile {
            agent: AgentSection {
                send_interval: 10,
                send_model: SendModel::default(),
                data_type: DataType::default(),
            },
            server: ServerSection {
                push: SectionPushModel {
                    url: "https://example.com".to_string(),
                    retries_connection: None,
                    url_params: vec![],
                    http_headers: vec![],
                },
                pull: SectionPullModel {
                    route: "metrics".to_string(),
                    host: "localhost".to_string(),
                },
            },
            metrics: MetricsConfig {
                processes: ProcessListConfig {
                    settings: CommonMetricSetting::default(),
                    process_limit: 5,
                    remove_dead_processes: true,
                    sort_by: ProcessSortBy::default(),
                },
                cpu: CpuConfig {
                    settings: CommonMetricSetting::default(),
                },
                disks: DisksConfig {
                    settings: CommonMetricSetting::default(),
                    ignore_fs_types: vec![String::from("tmpfs")],
                    ignore_mount_points: vec![String::from("/mnt/backup_test")],
                },
                network: NetworkConfig {
                    settings: CommonMetricSetting::default(),
                    list_type: ListType::default(),
                    interfaces: vec![String::from("lo"), String::from("docker0")],
                },
                system: SystemConfig {
                    settings: CommonMetricSetting::default(),
                },
                components: ComponentsConfig {
                    settings: CommonMetricSetting::default(),
                },
                memory: MemoryConfig {
                    settings: CommonMetricSetting::default(),
                },
            },
        }
    }
}

impl Default for CommonMetricSetting {
    fn default() -> Self {
        CommonMetricSetting {
            enabled: true,
            override_interval: None,
            override_retries_connection: None,
        }
    }
}
