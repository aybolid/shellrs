use std::collections::HashMap;

use is_executable::IsExecutable;

use super::{Command, EchoCommand, ExitCommand, ExternalCommand, TypeCommand};

pub struct CommandsRegistry {
    builtin: HashMap<String, Box<dyn Command>>,
    external: HashMap<String, Box<dyn Command>>,
}

impl CommandsRegistry {
    pub fn new() -> Self {
        Self {
            builtin: HashMap::new(),
            external: HashMap::new(),
        }
    }

    pub fn register_builtin(&mut self, command: Box<dyn Command>) {
        let name = command.get_name();
        assert!(
            !self.builtin.contains_key(&name),
            "duplicate builtin command: {}",
            name
        );
        self.builtin.insert(name, command);
    }

    pub fn get_command(&self, name: &str) -> Option<&Box<dyn Command>> {
        self.builtin.get(name).or_else(|| self.external.get(name))
    }

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

impl Default for CommandsRegistry {
    fn default() -> Self {
        let mut registry = Self::new();

        registry.register_builtin(Box::new(ExitCommand));
        registry.register_builtin(Box::new(EchoCommand));
        registry.register_builtin(Box::new(TypeCommand));

        registry.register_external();

        registry
    }
}
