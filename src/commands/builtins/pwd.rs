use crate::commands::{Command, CommandsRegistry};

pub struct PwdCommand;

impl Command for PwdCommand {
    fn run(&self, _: Vec<&str>, _: &CommandsRegistry) -> Result<(), String> {
        let pwd = std::env::current_dir().map_err(|err| err.to_string())?;
        println!("{}", pwd.display());
        Ok(())
    }

    fn get_name(&self) -> String {
        "pwd".to_string()
    }

    fn get_type_message(&self) -> String {
        format!("{} is a shell builtin", self.get_name())
    }
}
