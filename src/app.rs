use std::io::Write;

use levenshtein::Levenshtein;
use thiserror::Error;

use crate::{commands::CommandsRegistry, dprintln, dprintln_err};

#[derive(Debug, Error)]
pub enum ShellError {
    /// The input was empty. Not really an error btw.
    #[error("empty input")]
    EmptyInput,
    /// The command with the given name was not found.
    /// This error will trigger a suggestion for the closest command name using Levenshtein distance.
    #[error("{command_name}: command not found")]
    CommandNotFound { command_name: String },
    /// The command execution failed.
    /// The message will be formatted as an error message (red color).
    #[error("\x1b[31m{0}\x1b[0m")]
    CommandExecutionFail(String),
}

pub struct Shell {
    stdout: std::io::Stdout,
    stdin: std::io::Stdin,

    input_buffer: String,

    /// Registry of all registered commands (builtin and external).
    cmd_registry: CommandsRegistry,
}

impl Shell {
    /// Creates a new instance of the `Shell` struct.
    pub fn new() -> Self {
        Self {
            stdout: std::io::stdout(),
            stdin: std::io::stdin(),

            input_buffer: String::new(),

            cmd_registry: CommandsRegistry::default(),
        }
    }

    /// Runs the shell REPL (Read-Eval-Print-Loop).
    ///
    /// This version allows multiline input: if a line ends with an unescaped backslash,
    /// the input will be continued on the next line.
    pub fn run_repl(&mut self) {
        dprintln!("starting repl");
        loop {
            self.input_buffer = self.read_multiline_input();

            if let Err(err) = self.eval() {
                match err {
                    ShellError::EmptyInput => {
                        dprintln_err!("empty input error");
                    }
                    ShellError::CommandNotFound { command_name } => {
                        eprintln!("{}: command not found", command_name);

                        if let Some(closest_name) = Levenshtein::get_closest_with_threshold(
                            &command_name,
                            &self.cmd_registry.registered_names,
                            2,
                        ) {
                            eprintln!("did you mean \"{}\"?", closest_name);
                        }
                    }
                    _ => eprintln!("{}", err),
                }
            }

            self.input_buffer.clear();
        }
    }

    /// Reads multiline input from the user. A line ending with an unescaped backslash (`\`)
    /// indicates that the command continues on the next line.
    fn read_multiline_input(&mut self) -> String {
        println!(
            "\n\x1b[1;32m{}\x1b[0m", // bold, green
            std::env::current_dir().unwrap().display()
        );
        print!("> ");
        self.stdout.flush().unwrap();

        let mut complete_input = String::new();

        loop {
            let mut line = String::new();
            self.stdin.read_line(&mut line).unwrap();

            let line = line.trim_end_matches('\n').to_string();

            if line.ends_with('\\') && !line.ends_with("\\\\") {
                let line_without_bs = line.trim_end_matches('\\');
                complete_input.push_str(line_without_bs);
                complete_input.push(' ');

                print!("> ");
                self.stdout.flush().unwrap();
            } else {
                complete_input.push_str(&line);
                break;
            }
        }

        complete_input
    }

    /// Evaluates the given input string.
    fn eval(&mut self) -> Result<(), ShellError> {
        dprintln!("eval: {:?}", self.input_buffer);

        let tokens = self.parse_shell_input();
        dprintln!("parsed tokens: {:?}", tokens);

        if tokens.is_empty() {
            return Err(ShellError::EmptyInput);
        }

        let command_name = &tokens[0];
        let args: Vec<&str> = tokens[1..].iter().map(|s| s.as_str()).collect();

        dprintln!("command name: {}", command_name);
        dprintln!("args: {:?}", args);

        if let Some(command) = self.cmd_registry.get_command(command_name) {
            command.run(args, &self.cmd_registry)?;
        } else {
            return Err(ShellError::CommandNotFound {
                command_name: command_name.to_string(),
            });
        }

        Ok(())
    }

    /// Parse a shell-like input string into a vector of tokens.
    ///
    /// This function handles:
    /// - Whitespace-separated tokens.
    /// - Single (`'`) and double (`"`) quoted segments that allow spaces.
    /// - Escaped characters via a backslash (`\`).
    fn parse_shell_input(&self) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();

        // Flags to track whether we're inside single or double quotes.
        let mut in_single_quote = false;
        let mut in_double_quote = false;

        let mut chars = self.input_buffer.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                // toggle single-quote state (only when not inside double quotes)
                '\'' if !in_double_quote => {
                    in_single_quote = !in_single_quote;
                }
                // toggle double-quote state (only when not inside single quotes)
                '"' if !in_single_quote => {
                    in_double_quote = !in_double_quote;
                }
                // handle escape character: add the next character literally.
                '\\' => {
                    if let Some(escaped_char) = chars.next() {
                        current.push(escaped_char);
                    }
                }
                // if a space or tab is encountered outside quotes, finish the current token.
                ' ' | '\t' if !in_single_quote && !in_double_quote => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    // skip any additional consecutive whitespaces.
                    while let Some(&next_char) = chars.peek() {
                        if next_char == ' ' || next_char == '\t' {
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }
                // all other characters are added to the current token.
                _ => {
                    current.push(c);
                }
            }
        }

        if !current.is_empty() {
            tokens.push(current);
        }

        tokens
    }
}
