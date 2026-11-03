mod container;
mod cgroup;
mod filesystem;
mod namespace;

use container::{Container, ContainerConfig, ContainerState};
use std::ffi::{OsStr, OsString};
use std::io;
use std::path::PathBuf;

fn print_banner() {
    println!(
        r#"
╭──────────────────────────────────────────╮
│                                          │
│   ██████╗ ██╗   ██╗███████╗████████╗     │
│   ██╔══██╗██║   ██║██╔════╝╚══██╔══╝     │
│   ██████╔╝██║   ██║███████╗   ██║        │
│   ██╔══██╗██║   ██║╚════██║   ██║        │
│   ██║  ██║╚██████╔╝███████║   ██║        │
│   ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝        │
│                                          │
│      RustBox Container Runtime           │
│      Linux isolation from scratch        │
│                                          │
╰──────────────────────────────────────────╯
"#
    );
}

fn main() -> io::Result<()> {
    let mut args = std::env::args_os().skip(1);
    if args.next().as_deref() == Some(OsStr::new("--rustbox-manager")) {
        let rootfs = args
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "rootfs manquant"))?;
        let hostname = args
            .next()
            .and_then(|value| value.into_string().ok())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "hostname manquant"))?;
        if args.next().as_deref() != Some(OsStr::new("--")) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "séparateur de commande manquant",
            ));
        }
        let command = args
            .map(|value: OsString| {
                value.into_string().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "argument de commande non UTF-8",
                    )
                })
            })
            .collect::<io::Result<Vec<_>>>()?;
        std::process::exit(container::run_manager(rootfs, hostname, command)?);
    }

    print_banner();

    let config = ContainerConfig::new(
        PathBuf::from("./alpine"),
        String::from("rustbox"),
        Some(2 * 1024 * 1024 * 1024),
        Some(2),
        vec![
            String::from("/bin/sh"),
            String::from("-c"),
            String::from("echo /* && hostname && echo PID: $$ && cat /proc/1/status"),
        ],
    );
    let mut container = Container::new(config).with_name("demo");
    container.start()?;
    if let ContainerState::Exited(code) = container.state() {
        if *code != 0 {
            std::process::exit(*code);
        }
    }

    Ok(())
}
