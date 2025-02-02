use crate::{
    app::{Shell, ShellError},
    commands::Command,
};

#[derive(Debug)]
pub struct HelpCommand;

impl Command for HelpCommand {
    fn run(&self, args: Vec<&str>, shell: &mut Shell) -> Result<(), ShellError> {
        let command_name = match args.get(0) {
            Some(arg) => arg,
            None => {
                return Err(ShellError::CommandExecutionFail(
                    "example usage: help <command name>".to_string(),
                ))
            }
        };

        if let Some(command) = shell.cmd_registry.get_command(command_name) {
            let message = &command.clone().get_help_message(shell)?;
            shell.stdout.writeln(message);
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

    fn get_help_message(&self, _: &mut Shell) -> Result<String, ShellError> {
        let mut help_message = String::new();

        help_message.push_str(format!("usage: {} <command name>\n", self.get_name()).as_str());
        help_message.push_str("displays the help message for the specified command.\n");
        help_message.push_str("if target is an external command, the man command is used to display the help message.");

        Ok(help_message)
    }
}
