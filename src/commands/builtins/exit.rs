use crate::{
    app::{ShellError, ShellOutput},
    commands::{Command, CommandsRegistry},
    dprintln,
};

#[derive(Debug)]
pub struct ExitCommand;

impl Command for ExitCommand {
    fn run(
        &self,
        _: &mut ShellOutput,
        args: Vec<&str>,
        _: &CommandsRegistry,
    ) -> Result<(), ShellError> {
        let status_code_parse = match args.get(0) {
            Some(arg) => arg.parse::<i32>(),
            None => Ok(0),
        };

        match status_code_parse {
            Ok(status_code) => {
                dprintln!("exiting with status code {}", status_code);
                std::process::exit(status_code);
            }
            Err(err) => {
                return Err(ShellError::CommandExecutionFail(err.to_string()));
            }
        }
    }

    fn get_name(&self) -> String {
        "exit".to_string()
    }

    fn get_help_message(
        &self,
        _: &mut ShellOutput,
        _: &CommandsRegistry,
    ) -> Result<String, ShellError> {
        let mut help_message = String::new();

        help_message.push_str(format!("usage: {} <status code>\n", self.get_name()).as_str());
        help_message.push_str("exits the shell with the specified status code.\n");
        help_message.push_str("if no status code is specified, the exit status is 0.\n\n");
        help_message.push_str("panics if the status code parsing fails.");

        Ok(help_message)
    }
}
