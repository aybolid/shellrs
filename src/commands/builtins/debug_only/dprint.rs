use crate::commands::{Command, CommandsRegistry};

#[derive(Debug)]
pub struct DebugPrintCommand;

impl Command for DebugPrintCommand {
    fn run(&self, args: Vec<&str>, reg: &CommandsRegistry) -> Result<(), String> {
        if args.is_empty() {
            return Err("example usage: dprint <command name>".to_string());
        }

        let command_name = args[0];

        if let Some(command) = reg.get_command(command_name) {
            command.debug_print();
        } else {
            return Err(format!("{}: not found", command_name));
        }

        Ok(())
    }

    fn get_name(&self) -> String {
        "dprint".to_string()
    }

    fn get_type_message(&self) -> String {
        format!("{} is a debug-only shell builtin", self.get_name())
    }
}
