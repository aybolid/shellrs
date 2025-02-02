mod registry;
pub use registry::CommandsRegistry;

mod builtins;
pub use builtins::*;

mod external;
pub use external::ExternalCommand;

use crate::app::{Shell, ShellError};

pub trait Command
where
    Self: std::fmt::Debug,
{
    /// Executes the command with the given arguments.
    fn run(&self, args: Vec<&str>, shell: &mut Shell) -> Result<(), ShellError>;

    /// Returns the name of the command.
    fn get_name(&self) -> String;

    /// Returns a message describing the help of the command.
    fn get_help_message(&self, shell: &mut Shell) -> Result<String, ShellError>;

    #[cfg(debug_assertions)]
    fn debug_print_message(&self) -> String {
        format!("{:?}", self)
    }
}
