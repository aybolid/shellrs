use crate::{
    app::ShellError,
    commands::{Command, CommandsRegistry},
};

#[derive(Debug)]
pub struct TypeCommand;

impl Command for TypeCommand {
    fn run(&self, args: Vec<&str>, reg: &CommandsRegistry) -> Result<(), ShellError> {
        let command_name = match args.get(0) {
            Some(arg) => arg,
            None => {
                return Err(ShellError::CommandExecutionFail(
                    "example usage: type <command name>".to_string(),
                ))
            }
        };

        if let Some(command) = reg.get_command(command_name) {
            println!("{}", command.get_type_message());
        } else {
            println!("{}: not found", command_name);
        }

        Ok(())
    }

    fn get_name(&self) -> String {
        "type".to_string()
    }

    fn get_type_message(&self) -> String {
        format!("{} is a shell builtin", self.get_name())
    }
}
