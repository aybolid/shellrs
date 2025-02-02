use crate::{
    app::{ShellError, ShellOutput},
    dprintln,
};

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
    fn run(
        &self,
        _: &mut ShellOutput,
        args: Vec<&str>,
        _: &CommandsRegistry,
    ) -> Result<(), ShellError> {
        dprintln!("spawning external command: {}", self.debug_print_message());

        let mut child = std::process::Command::new(&self.path)
            .args(args)
            .spawn()
            .map_err(|err| ShellError::CommandExecutionFail(err.to_string()))?;

        child
            .wait()
            .map_err(|err| ShellError::CommandExecutionFail(err.to_string()))?;

        Ok(())
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_help_message(
        &self,
        out: &mut ShellOutput,
        reg: &CommandsRegistry,
    ) -> Result<String, ShellError> {
        dprintln!(
            "trying to run man for external command: {}",
            self.get_name()
        );

        if let Some(man_cmd) = reg.get_command("man") {
            man_cmd.run(out, vec![&self.get_name()], reg)?;
            return Ok("".to_string());
        } else {
            return Err(ShellError::CommandExecutionFail(
                "no man command found. can't display help message for external command".to_string(),
            ));
        }
    }
}
