mod registry;
pub use registry::CommandsRegistry;

mod builtins;
pub use builtins::*;

mod external;
pub use external::ExternalCommand;

pub trait Command
where
    Self: std::fmt::Debug,
{
    /// Executes the command with the given arguments.
    fn run(&self, args: Vec<&str>, reg: &CommandsRegistry) -> Result<(), String>;

    /// Returns the name of the command.
    fn get_name(&self) -> String;

    /// Returns a message describing the type of the command.
    /// Used by the `type` builtin command.
    fn get_type_message(&self) -> String;

    fn debug_print(&self) {
        println!("{:?}", self);
    }
}
