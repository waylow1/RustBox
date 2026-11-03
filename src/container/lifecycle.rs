use super::process::ContainerConfig;
use crate::{cgroup::ContainerCgroup, filesystem, namespace};
use std::ffi::CString;
use std::io;
use std::os::fd::AsRawFd;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug)]
pub enum ContainerState {
    Created,
    Running,
    Exited(i32),
}

pub struct Container {
    name: String,
    config: ContainerConfig,
    state: ContainerState,
}

impl Container {
    pub fn new(config: ContainerConfig) -> Self {
        Self {
            name: String::from("default"),
            config,
            state: ContainerState::Created,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn state(&self) -> &ContainerState {
        &self.state
    }

    pub fn start(&mut self) -> io::Result<()> {
        self.config.validate()?;
        let executable = std::env::current_exe()?;
        let cgroup = ContainerCgroup::create(self.config.cpu_limit(), self.config.memory_limit())?;
        let cgroup_procs = match cgroup
            .as_ref()
            .map(|cgroup| cgroup.procs().try_clone())
            .transpose()
        {
            Ok(procs) => procs,
            Err(error) => return cleanup_cgroup(Err(error), cgroup),
        };

        let mut command = Command::new(executable);
        command
            .arg("--rustbox-manager")
            .arg(self.config.rootfs())
            .arg(self.config.hostname())
            .arg("--")
            .args(self.config.command());
        unsafe {
            command.pre_exec(move || {
                if let Some(procs) = &cgroup_procs {
                    ContainerCgroup::attach_current_process_fd(procs.as_raw_fd())?;
                }
                Ok(())
            });
        }

        self.state = ContainerState::Running;
        println!("[rustbox] conteneur « {} » démarré", self.name);
        let status = command.status();
        drop(command);
        let status = match cleanup_cgroup(status, cgroup) {
            Ok(status) => status,
            Err(error) => {
                self.state = ContainerState::Exited(1);
                return Err(error);
            }
        };
        let code = status.code().unwrap_or(1);
        self.state = ContainerState::Exited(code);
        println!(
            "[rustbox] conteneur « {} » terminé (code {code})",
            self.name
        );

        Ok(())
    }
}

pub fn run_manager(rootfs: PathBuf, hostname: String, command: Vec<String>) -> io::Result<i32> {
    if command.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "la commande du conteneur est vide",
        ));
    }

    run_container_process(rootfs, hostname, command)
}

fn run_container_process(
    rootfs: PathBuf,
    hostname: String,
    command: Vec<String>,
) -> io::Result<i32> {
    namespace::enter_container_namespaces()?;
    namespace::set_hostname(&hostname)?;
    filesystem::make_mounts_private()?;
    filesystem::enter_rootfs(&rootfs)?;

    let proc_source = CString::new("proc").expect("static string has no null byte");
    let proc_target = CString::new("/proc").expect("static string has no null byte");
    let mut child = Command::new(&command[0]);
    child.args(&command[1..]);
    unsafe {
        child.pre_exec(move || {
            let ret = libc::mount(
                proc_source.as_ptr(),
                proc_target.as_ptr(),
                proc_source.as_ptr(),
                0,
                std::ptr::null(),
            );
            if ret == -1 {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        });
    }

    let status = child.status();
    let status = status?;
    drop(child);
    Ok(status.code().unwrap_or(1))
}

fn cleanup_cgroup<T>(result: io::Result<T>, cgroup: Option<ContainerCgroup>) -> io::Result<T> {
    let cleanup = cgroup.map_or(Ok(()), ContainerCgroup::cleanup);

    match (result, cleanup) {
        (Ok(value), Ok(())) => Ok(value),
        (Err(error), Ok(())) => Err(error),
        (Ok(_), Err(error)) => Err(error),
        (Err(error), Err(cleanup_error)) => Err(io::Error::new(
            error.kind(),
            format!("{error}; nettoyage du cgroup impossible : {cleanup_error}"),
        )),
    }
}
