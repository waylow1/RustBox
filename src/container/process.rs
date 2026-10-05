use std::path::PathBuf;
pub struct ContainerConfig {
    rootfs: PathBuf,
    hostname: String,
    memory_limit: Option<u64>,
    cpu_limit: Option<u64>,
    command: Vec<String>,
}

impl ContainerConfig {
    pub fn new(
        rootfs: PathBuf,
        hostname: String,
        memory_limit: Option<u64>,
        cpu_limit: Option<u64>,
        command: Vec<String>,
    ) -> Self {
        Self {
            rootfs,
            hostname,
            memory_limit,
            cpu_limit,
            command,
        }
    }

    pub fn validate(&self) -> std::io::Result<()> {
        if self.command.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "la commande du conteneur est vide",
            ));
        }
        if !self.rootfs.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("rootfs introuvable : {}", self.rootfs.display()),
            ));
        }
        Ok(())
    }

    pub fn rootfs(&self) -> &PathBuf {
        &self.rootfs
    }

    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    pub fn command(&self) -> &[String] {
        &self.command
    }
}
