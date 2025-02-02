use crate::{
    app::{Shell, ShellError},
    commands::Command,
};

#[derive(Debug)]
pub struct EchoCommand;

impl Command for EchoCommand {
    fn run(&self, args: Vec<&str>, shell: &mut Shell) -> Result<(), ShellError> {
        shell.stdout.writeln(&args.join(" "));
        Ok(())
    }

    fn get_name(&self) -> String {
        "echo".to_string()
    }

    fn get_help_message(&self, _: &mut Shell) -> Result<String, ShellError> {
        let mut help_message = String::new();

        help_message.push_str(format!("usage: {} <message>\n", self.get_name()).as_str());
        help_message.push_str("prints the specified message to the standard output.");

        Ok(help_message)
    }
}
