mod registry;
pub use registry::CommandsRegistry;

mod builtins;
pub use builtins::*;

mod external;
pub use external::ExternalCommand;

pub trait Command {
    fn run(&self, args: Vec<&str>, reg: &CommandsRegistry) -> Result<(), String>;
    fn get_name(&self) -> String;
    fn get_type_message(&self) -> String;
}
