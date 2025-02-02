use crate::{
    commands::{Command, CommandsRegistry},
    dprintln,
};

#[derive(Debug)]
pub struct CdCommand;

impl Command for CdCommand {
    fn run(&self, args: Vec<&str>, _: &CommandsRegistry) -> Result<(), String> {
        let target_dir = match args.get(0) {
            Some(arg) => arg.to_string(),
            None => std::env::var("HOME").unwrap_or_else(|_| "/".to_string()),
        };

        let target_dir = std::path::Path::new(&target_dir);
        dprintln!("changing directory to {:?}", target_dir);

        std::env::set_current_dir(target_dir).map_err(|err| err.to_string())?;

        Ok(())
    }

    fn get_name(&self) -> String {
        "cd".to_string()
    }

    fn get_type_message(&self) -> String {
        format!("{} is a shell builtin", self.get_name())
    }
}
