use crate::{
    app::{ShellError, ShellOutput},
    commands::{Command, CommandsRegistry},
};

#[derive(Debug)]
pub struct EchoCommand;

impl Command for EchoCommand {
    fn run(
        &self,
        out: &mut ShellOutput,
        args: Vec<&str>,
        _: &CommandsRegistry,
    ) -> Result<(), ShellError> {
        out.writeln(&args.join(" "));
        Ok(())
    }

    fn get_name(&self) -> String {
        "echo".to_string()
    }

    fn get_help_message(
        &self,
        _: &mut ShellOutput,
        _: &CommandsRegistry,
    ) -> Result<String, ShellError> {
        let mut help_message = String::new();

        help_message.push_str(format!("usage: {} <message>\n", self.get_name()).as_str());
        help_message.push_str("prints the specified message to the standard output.");

        Ok(help_message)
    }
}
