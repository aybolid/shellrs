use crate::commands::{Command, CommandsRegistry};

#[derive(Debug)]
pub struct ExitCommand;

impl Command for ExitCommand {
    fn run(&self, args: Vec<&str>, _: &CommandsRegistry) -> Result<(), String> {
        let status_code = match args.get(0) {
            Some(arg) => arg.parse::<i32>().unwrap_or(0),
            None => 0,
        };
        std::process::exit(status_code);
    }

    fn get_name(&self) -> String {
        "exit".to_string()
    }

    fn get_type_message(&self) -> String {
        format!("{} is a shell builtin", self.get_name())
    }
}
