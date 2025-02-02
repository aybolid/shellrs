use crate::{
    app::{ShellError, ShellOutput},
    commands::{Command, CommandsRegistry},
};

#[derive(Debug)]
pub struct HelpCommand;

impl Command for HelpCommand {
    fn run(
        &self,
        out: &mut ShellOutput,
        args: Vec<&str>,
        reg: &CommandsRegistry,
    ) -> Result<(), ShellError> {
        let command_name = match args.get(0) {
            Some(arg) => arg,
            None => {
                return Err(ShellError::CommandExecutionFail(
                    "example usage: help <command name>".to_string(),
                ))
            }
        };

        if let Some(command) = reg.get_command(command_name) {
            let message = &command.get_help_message(out, reg)?;
            out.writeln(message);
        } else {
            return Err(ShellError::CommandNotFound {
                command_name: command_name.to_string(),
            });
        }

        Ok(())
    }

    fn get_name(&self) -> String {
        "help".to_string()
    }

    fn get_help_message(
        &self,
        _: &mut ShellOutput,
        _: &CommandsRegistry,
    ) -> Result<String, ShellError> {
        let mut help_message = String::new();

        help_message.push_str(format!("usage: {} <command name>\n", self.get_name()).as_str());
        help_message.push_str("displays the help message for the specified command.\n");
        help_message.push_str("if target is an external command, the man command is used to display the help message.");

        Ok(help_message)
    }
}
