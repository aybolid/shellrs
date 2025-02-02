use std::collections::HashMap;

use is_executable::IsExecutable;

use crate::dprintln;

use super::{
    CdCommand, Command, EchoCommand, ExitCommand, ExternalCommand, PwdCommand, TypeCommand,
};

#[cfg(debug_assertions)]
use super::DebugPrintCommand;

pub struct CommandsRegistry {
    /// Registry of builtin commands.
    /// The key is the command name and the value is the command itself.
    builtin: HashMap<String, Box<dyn Command>>,
    /// Registry of external commands.
    /// The key is the command name and the value is the command itself.
    external: HashMap<String, Box<dyn Command>>,
}

impl CommandsRegistry {
    /// Creates a new instance of the `CommandsRegistry` struct.
    /// Initializes the registry hash maps for builtin and external commands.
    pub fn new() -> Self {
        Self {
            builtin: HashMap::new(),
            external: HashMap::new(),
        }
    }

    /// Returns a reference to the `Command` with the given name if it exists.
    /// First checks builtin commands, then external commands.
    pub fn get_command(&self, name: &str) -> Option<&Box<dyn Command>> {
        self.builtin.get(name).or_else(|| self.external.get(name))
    }

    /// Registers a new `Command`.
    /// Panics if a command with the same name already exists.
    pub fn register_builtin(&mut self, command: Box<dyn Command>) {
        let name = command.get_name();

        assert!(
            !self.builtin.contains_key(&name),
            "duplicate builtin command: {}",
            name
        );
        dprintln!("loading builtin command: {}", name,);

        self.builtin.insert(name, command);
    }

    /// Registers all external commands in the `PATH` environment variable.
    pub fn register_external(&mut self) {
        if let Ok(paths) = std::env::var("PATH") {
            for dir in paths.split(':') {
                dprintln!("loading external commands from {}", dir);

                #[cfg(debug_assertions)]
                let mut count: usize = 0;

                if let Ok(entries) = std::fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if !path.is_executable() {
                            continue;
                        }

                        let name = match entry.file_name().into_string() {
                            Ok(name) => name,
                            Err(_) => continue,
                        };

                        if self.external.contains_key(&name) {
                            continue;
                        }

                        let executable_path = path.to_string_lossy().to_string();
                        let external_command = ExternalCommand::new(name.clone(), executable_path);

                        self.external
                            .insert(external_command.get_name(), Box::new(external_command));

                        #[cfg(debug_assertions)]
                        {
                            count += 1;
                        }
                    }
                } else {
                    dprintln!("failed to read external commands from {}", dir);
                }

                dprintln!("loaded {} external commands", count);
            }
        } else {
            dprintln!("PATH environment variable not set");
        }
    }
}

macro_rules! register_builtins {
    ($registry:expr, $( $cmd:expr ),* ) => {
        $( $registry.register_builtin(Box::new($cmd)); )*
    };
}

impl Default for CommandsRegistry {
    /// Creates a new instance of the `CommandsRegistry` struct and loads builtin and external commands.
    /// Some builtin commands are only available in debug builds.
    fn default() -> Self {
        let mut registry = Self::new();

        #[cfg(debug_assertions)] // only available in debug builds
        {
            register_builtins!(registry, DebugPrintCommand);
        }

        register_builtins!(
            registry,
            ExitCommand,
            EchoCommand,
            TypeCommand,
            PwdCommand,
            CdCommand
        );

        registry.register_external();

        registry
    }
}
