use crate::commands::{Command, CommandsRegistry};

pub struct AllCommand;

impl Command for AllCommand {
    fn run(&self, _: Vec<&str>, reg: &CommandsRegistry) -> Result<(), String> {
        let (mut builtin_names, mut external_names) = reg.get_all_registered_names();

        println!("builtin commands ({}):", builtin_names.len());
        builtin_names.sort();
        println!("  {}", builtin_names.join(", "));

        println!("external commands ({}):", external_names.len());
        external_names.sort();
        println!("  {}", external_names.join(", "));

        Ok(())
    }

    fn get_name(&self) -> String {
        "all".to_string()
    }

    fn get_type_message(&self) -> String {
        format!("{} is a shell builtin", self.get_name())
    }
}
