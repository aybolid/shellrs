use crate::dprintln;

use super::{Command, CommandsRegistry};

#[derive(Debug)]
pub struct ExternalCommand {
    /// The name of the command.
    name: String,
    /// Full path of the command.
    path: String,
}

impl ExternalCommand {
    pub fn new(name: String, path: String) -> Self {
        Self { name, path }
    }
}

impl Command for ExternalCommand {
    fn run(&self, args: Vec<&str>, _: &CommandsRegistry) -> Result<(), String> {
        dprintln!("spawning external command: {}", self.debug_print_message());
        let mut child = std::process::Command::new(&self.name)
            .args(args)
            .spawn()
            .map_err(|err| err.to_string())?;

        child.wait().map_err(|err| err.to_string())?;

        Ok(())
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type_message(&self) -> String {
        format!("{} is {}", self.get_name(), self.path)
    }
}
