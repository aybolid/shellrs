use crate::{
    app::{Shell, ShellError},
    commands::Command,
};

#[derive(Debug)]
pub struct DebugPrintCommand;

impl Command for DebugPrintCommand {
    fn run(&self, args: Vec<&str>, shell: &mut Shell) -> Result<(), ShellError> {
        if args.is_empty() {
            return Err(ShellError::CommandExecutionFail(
                "example usage: dprint <command name>".to_string(),
            ));
        }

        let command_name = args[0];

        if let Some(command) = shell.cmd_registry.get_command(command_name) {
            shell.stdout.writeln(&command.debug_print_message());
        } else {
            return Err(ShellError::CommandNotFound {
                command_name: command_name.to_string(),
            });
        }

        Ok(())
    }

    fn get_name(&self) -> String {
        "dprint".to_string()
    }

    fn get_help_message(&self, _: &mut Shell) -> Result<String, ShellError> {
        let mut help_message = String::new();

        help_message.push_str(format!("usage: {} <command name>\n", self.get_name()).as_str());
        help_message.push_str("debug prints help message of the specified command.");

        Ok(help_message)
    }
}
