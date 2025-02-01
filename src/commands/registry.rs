use std::collections::HashMap;

use is_executable::IsExecutable;

use crate::commands::CdCommand;

use super::{
    AllCommand, Command, EchoCommand, ExitCommand, ExternalCommand, PwdCommand, TypeCommand,
};

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

    /// Returns a tuple of two vectors containing the names of all registered builtin and external commands.
    /// `(builtin_names, external_names)`
    pub fn get_all_registered_names(&self) -> (Vec<String>, Vec<String>) {
        let builtin_names = self
            .builtin
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<_>>();

        let external_names = self
            .external
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<_>>();

        (builtin_names, external_names)
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
        self.builtin.insert(name, command);
    }

    /// Registers all external commands in the `PATH` environment variable.
    pub fn register_external(&mut self) {
        if let Ok(paths) = std::env::var("PATH") {
            for dir in paths.split(':') {
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
                    }
                }
            }
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
    fn default() -> Self {
        let mut registry = Self::new();

        register_builtins!(
            registry,
            ExitCommand,
            EchoCommand,
            TypeCommand,
            AllCommand,
            PwdCommand,
            CdCommand
        );

        registry.register_external();

        registry
    }
}
