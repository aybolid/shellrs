use crate::{
    app::{ShellError, ShellOutput},
    commands::{Command, CommandsRegistry},
};

#[derive(Debug)]
pub struct PwdCommand;

impl Command for PwdCommand {
    fn run(
        &self,
        out: &mut ShellOutput,
        _: Vec<&str>,
        _: &CommandsRegistry,
    ) -> Result<(), ShellError> {
        let pwd = std::env::current_dir()
            .map_err(|err| ShellError::CommandExecutionFail(err.to_string()))?;

        out.writeln(&pwd.display().to_string());

        Ok(())
    }

    fn get_name(&self) -> String {
        "pwd".to_string()
    }

    fn get_help_message(
        &self,
        _: &mut ShellOutput,
        _: &CommandsRegistry,
    ) -> Result<String, ShellError> {
        let mut help_message = String::new();

        help_message.push_str(format!("usage: {}\n", self.get_name()).as_str());
        help_message.push_str("prints the current working directory.");

        Ok(help_message)
    }
}
