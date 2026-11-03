use std::fs::{self, File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

pub mod cpu;
pub mod memory;

pub use cpu::CpuCgroup;
pub use memory::MemoryCgroup;

const CGROUP_ROOT: &str = "/sys/fs/cgroup";
static NEXT_CGROUP_ID: AtomicU64 = AtomicU64::new(0);

pub struct ContainerCgroup {
    path: PathBuf,
    procs: File,
}

impl ContainerCgroup {
    pub fn create(cpu_cores: Option<u64>, memory_bytes: Option<u64>) -> io::Result<Option<Self>> {
        if cpu_cores.is_none() && memory_bytes.is_none() {
            return Ok(None);
        }

        let base = Path::new(CGROUP_ROOT).join("rustbox");
        fs::create_dir_all(&base)?;

        let mut controllers = Vec::new();
        if cpu_cores.is_some() {
            controllers.push("cpu");
        }
        if memory_bytes.is_some() {
            controllers.push("memory");
        }

        let available = fs::read_to_string(base.join("cgroup.controllers"))?;
        for controller in &controllers {
            if !available.split_whitespace().any(|item| item == *controller) {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    format!(
                        "le contrôleur cgroup « {controller} » n'est pas disponible sous {}",
                        base.display()
                    ),
                ));
            }
        }

        let enabled = controllers
            .iter()
            .map(|controller| format!("+{controller}"))
            .collect::<Vec<_>>()
            .join(" ");
        fs::write(base.join("cgroup.subtree_control"), enabled)?;

        let path = loop {
            let id = NEXT_CGROUP_ID.fetch_add(1, Ordering::Relaxed);
            let path = base.join(format!("container-{}-{id}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => break path,
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error),
            }
        };

        let configure_result = (|| {
            if let Some(cores) = cpu_cores {
                CpuCgroup::apply(&path, cores)?;
            }
            if let Some(bytes) = memory_bytes {
                MemoryCgroup::apply(&path, bytes)?;
            }
            OpenOptions::new()
                .write(true)
                .open(path.join("cgroup.procs"))
        })();

        match configure_result {
            Ok(procs) => Ok(Some(Self { path, procs })),
            Err(error) => {
                if let Err(cleanup_error) = fs::remove_dir(&path) {
                    return Err(io::Error::new(
                        error.kind(),
                        format!(
                            "{error}; impossible de nettoyer {} : {cleanup_error}",
                            path.display()
                        ),
                    ));
                }
                Err(error)
            }
        }
    }

    pub fn procs(&self) -> &File {
        &self.procs
    }

    pub fn attach_current_process_fd(cgroup_procs_fd: i32) -> io::Result<()> {
        let mut pid = unsafe { libc::getpid() } as u32;
        let mut buffer = [0u8; 11];
        let mut position = buffer.len() - 1;

        loop {
            position -= 1;
            buffer[position] = b'0' + (pid % 10) as u8;
            pid /= 10;
            if pid == 0 {
                break;
            }
        }
        buffer[buffer.len() - 1] = b'\n';
        let length = buffer.len() - position;
        let mut written = 0;
        while written < length {
            let result = unsafe {
                libc::write(
                    cgroup_procs_fd,
                    buffer[position + written..].as_ptr().cast(),
                    length - written,
                )
            };
            if result == -1 {
                return Err(io::Error::last_os_error());
            }
            if result == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "écriture du PID dans le cgroup incomplète",
                ));
            }
            written += result as usize;
        }
        Ok(())
    }

    pub fn cleanup(self) -> io::Result<()> {
        let Self { path, procs } = self;
        drop(procs);
        fs::remove_dir(path)
    }
}