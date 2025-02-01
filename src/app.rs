use std::io::Write;

use crate::commands::CommandsRegistry;

pub struct REPL {
    stdout: std::io::Stdout,
    stdin: std::io::Stdin,

    cmd_registry: CommandsRegistry,
}

impl REPL {
    pub fn new() -> Self {
        Self {
            stdout: std::io::stdout(),
            stdin: std::io::stdin(),

            cmd_registry: CommandsRegistry::default(),
        }
    }

    pub fn run(&mut self) {
        loop {
            print!("$ ");
            self.stdout.flush().unwrap();

            let mut input_buffer = String::new();
            self.stdin.read_line(&mut input_buffer).unwrap();

            if let Err(err) = self.eval(&input_buffer) {
                eprintln!("{}", err);
            }
        }
    }

    fn eval(&mut self, input: &str) -> Result<(), String> {
        if input.trim().is_empty() {
            return Ok(());
        }

        let tokens: Vec<&str> = input.split_whitespace().collect();
        let command_name = tokens[0];
        let args = &tokens[1..];

        if let Some(command) = self.cmd_registry.get_command(command_name) {
            command.run(args.to_vec(), &self.cmd_registry)?;
        } else {
            return Err(format!("{}: command not found", command_name));
        }

        Ok(())
    }
}
