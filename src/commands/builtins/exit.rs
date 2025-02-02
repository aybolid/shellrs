use crate::{
    app::ShellError,
    commands::{Command, CommandsRegistry},
    dprintln,
};

#[derive(Debug)]
pub struct ExitCommand;

impl Command for ExitCommand {
    fn run(&self, args: Vec<&str>, _: &CommandsRegistry) -> Result<(), ShellError> {
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

    fn get_type_message(&self) -> String {
        format!("{} is a shell builtin", self.get_name())
    }
}
