use crate::{
    app::ShellError,
    commands::{Command, CommandsRegistry},
};

#[derive(Debug)]
pub struct EchoCommand;

impl Command for EchoCommand {
    fn run(&self, args: Vec<&str>, _: &CommandsRegistry) -> Result<(), ShellError> {
        Ok(println!("{}", args.join(" ")))
    }

    fn get_name(&self) -> String {
        "echo".to_string()
    }

    fn get_type_message(&self) -> String {
        format!("{} is a shell builtin", self.get_name())
    }
}
