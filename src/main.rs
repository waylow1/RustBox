mod container;

use container::{Container, ContainerConfig};
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

fn main() {
    print_banner();

    let config = ContainerConfig::new(
        PathBuf::from("./alpine"),
        String::from("rustbox-demo"),
        None,
        None,
        vec![String::from("/bin/sh")],
    );

    let mut container = Container::new(config).with_name("demo");
    println!("Conteneur créé, état : {:?}", container.state());

    if let Err(e) = container.start() {
        eprintln!("[rustbox] impossible de démarrer : {e}");
        eprintln!("[rustbox] : créez un rootfs, ex. `mkdir alpine` + debootstrap/apk");
        return;
    }
    println!("Conteneur terminé, état : {:?}", container.state());
}
