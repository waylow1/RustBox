mod container;
mod namespace;
use container::{Container, ContainerConfig};
use std::path::PathBuf;
mod filesystem;

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

fn main() -> std::io::Result<()> {
    print_banner();

    namespace::enter_new_uts()?;
    namespace::set_hostname("rustbox")?;

    let rootfs = std::path::Path::new("./alpine");
    filesystem::enter_rootfs(rootfs)?;
    println!("Rootfs actif. Contenu de / via le shell :");

    std::process::Command::new("/bin/sh")
        .args(["-c", "echo /* && hostname"])
        .status()?;

    Ok(())
}
