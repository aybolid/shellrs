use crate::{
    app::{Shell, ShellError},
    dprintln,
};

use super::Command;

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
    fn run(&self, args: Vec<&str>, shell: &mut Shell) -> Result<(), ShellError> {
        dprintln!("spawning external command: {}", self.debug_print_message());

        let stdout_stdio = shell
            .stdout
            .as_stdio()
            .map_err(|err| ShellError::CommandExecutionFail(err.to_string()))?;

        let stderr_stdio = shell
            .stderr
            .as_stdio()
            .map_err(|err| ShellError::CommandExecutionFail(err.to_string()))?;

        let mut child = std::process::Command::new(&self.path)
            .args(args)
            .stdout(stdout_stdio)
            .stderr(stderr_stdio)
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

    fn get_help_message(&self, shell: &mut Shell) -> Result<String, ShellError> {
        dprintln!(
            "trying to run man for external command: {}",
            self.get_name()
        );

        let man_cmd = shell.cmd_registry.get_command("man");
        if let Some(man_cmd) = man_cmd {
            man_cmd.clone().run(vec![&self.get_name()], shell)?;
            return Ok("".to_string());
        } else {
            return Err(ShellError::CommandExecutionFail(
                "no man command found. can't display help message for external command".to_string(),
            ));
        }
    }
}
