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
        if self.memory_limit == Some(0) || self.cpu_limit == Some(0) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "les limites CPU et mémoire doivent être supérieures à zéro",
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

    pub fn memory_limit(&self) -> Option<u64> {
        self.memory_limit
    }

    pub fn cpu_limit(&self) -> Option<u64> {
        self.cpu_limit
    }

    pub fn command(&self) -> &[String] {
        &self.command
    }
}
