mod registry;
pub use registry::CommandsRegistry;

mod builtins;
pub use builtins::*;

mod external;
pub use external::ExternalCommand;

use crate::app::ShellError;

pub trait Command
where
    Self: std::fmt::Debug,
{
    /// Executes the command with the given arguments.
    fn run(&self, args: Vec<&str>, reg: &CommandsRegistry) -> Result<(), ShellError>;

    /// Returns the name of the command.
    fn get_name(&self) -> String;

    /// Returns a message describing the type of the command.
    /// Used by the `type` builtin command.
    fn get_type_message(&self) -> String;

    #[cfg(debug_assertions)]
    fn debug_print_message(&self) -> String {
        format!("{:?}\n\t{}", self, self.get_type_message())
    }
}
