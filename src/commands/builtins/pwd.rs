use crate::{
    app::ShellError,
    commands::{Command, CommandsRegistry},
};

#[derive(Debug)]
pub struct PwdCommand;

impl Command for PwdCommand {
    fn run(&self, _: Vec<&str>, _: &CommandsRegistry) -> Result<(), ShellError> {
        let pwd = std::env::current_dir()
            .map_err(|err| ShellError::CommandExecutionFail(err.to_string()))?;
        println!("{}", pwd.display());
        Ok(())
    }

    fn get_name(&self) -> String {
        "pwd".to_string()
    }

    fn get_type_message(&self) -> String {
        format!("{} is a shell builtin", self.get_name())
    }
}
