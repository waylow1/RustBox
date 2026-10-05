use super::process::ContainerConfig;
use std::io;
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

        self.state = ContainerState::Running;
        println!("[rustbox] conteneur « {} » démarré", self.name);

        let command = self.config.command();
        let status = Command::new(&command[0]).args(&command[1..]).status()?;

        let code = status.code().unwrap_or(1);
        self.state = ContainerState::Exited(code);
        println!(
            "[rustbox] conteneur « {} » terminé (code {code})",
            self.name
        );

        Ok(())
    }
}
