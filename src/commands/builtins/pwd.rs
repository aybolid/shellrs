use crate::{
    app::{Shell, ShellError},
    commands::Command,
};

#[derive(Debug)]
pub struct PwdCommand;

impl Command for PwdCommand {
    fn run(&self, _: Vec<&str>, shell: &mut Shell) -> Result<(), ShellError> {
        let pwd = std::env::current_dir()
            .map_err(|err| ShellError::CommandExecutionFail(err.to_string()))?;

        shell.stdout.writeln(&pwd.display().to_string());

        Ok(())
    }

    fn get_name(&self) -> String {
        "pwd".to_string()
    }

    fn get_help_message(&self, _: &mut Shell) -> Result<String, ShellError> {
        let mut help_message = String::new();

        help_message.push_str(format!("usage: {}\n", self.get_name()).as_str());
        help_message.push_str("prints the current working directory.");

        Ok(help_message)
    }
}
