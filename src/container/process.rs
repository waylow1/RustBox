use std::path::PathBuf;

pub struct ContainerConfig {
    rootfs: PathBuf,
    hostname: String,
    memory_limit: Option<u64>,
    cpu_limit: Option<u64>,
    command: Vec<String>,
}