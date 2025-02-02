use std::io::Write;

use thiserror::Error;

use crate::{commands::CommandsRegistry, dprintln, dprintln_err};

#[derive(Debug, Error)]
pub enum ShellError {
    #[error("empty input")]
    EmptyInput,
    #[error("{0}: command not found")]
    CommandNotFound(String),
    #[error("{0}")]
    CommandExecutionFail(String),
}

pub struct Shell {
    stdout: std::io::Stdout,
    stdin: std::io::Stdin,

    /// Registry of all registered commands (builtin and external).
    cmd_registry: CommandsRegistry,
}

impl Shell {
    /// Creates a new instance of the `Shell` struct.
    pub fn new() -> Self {
        Self {
            stdout: std::io::stdout(),
            stdin: std::io::stdin(),

            cmd_registry: CommandsRegistry::default(),
        }
    }

    /// Runs the shell REPL (Read-Eval-Print-Loop).
    pub fn run_repl(&mut self) {
        dprintln!("starting repl");
        loop {
            println!(
                "\n\x1b[1;32m{}\x1b[0m", // bold, green
                std::env::current_dir().unwrap().display()
            );
            print!("> ");
            self.stdout.flush().unwrap();

            let mut input_buffer = String::new();
            self.stdin.read_line(&mut input_buffer).unwrap();

            if let Err(err) = self.eval(&input_buffer) {
                match err {
                    ShellError::EmptyInput => {
                        dprintln_err!("empty input error");
                    }
                    _ => eprintln!("{}", err),
                }
            }
        }
    }

    /// Evaluates the given input string.
    fn eval(&mut self, input: &str) -> Result<(), ShellError> {
        dprintln!("eval: {:?}", input);

        if input.trim().is_empty() {
            return Err(ShellError::EmptyInput);
        }

        let tokens: Vec<&str> = input.split_whitespace().collect();
        let command_name = tokens[0];
        let args = &tokens[1..];

        dprintln!("command name: {}", command_name);
        dprintln!("args: {:?}", args);

        if let Some(command) = self.cmd_registry.get_command(command_name) {
            command.run(args.to_vec(), &self.cmd_registry)?;
        } else {
            return Err(ShellError::CommandNotFound(command_name.to_string()));
        }

        Ok(())
    }
}
