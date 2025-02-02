use std::{io::Write, process::Stdio};

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

/// The output type of the shell.
/// Can be either a standard output, a standard error or a file.
pub enum ShellOutput {
    Stdout(std::io::StdoutLock<'static>),
    Stderr(std::io::StderrLock<'static>),
    #[allow(dead_code)]
    File(std::fs::File),
}

impl ShellOutput {
    pub fn stdout() -> Self {
        ShellOutput::Stdout(std::io::stdout().lock())
    }

    pub fn stderr() -> Self {
        ShellOutput::Stderr(std::io::stderr().lock())
    }

    #[allow(dead_code)]
    pub fn file(path: String) -> Self {
        ShellOutput::File(std::fs::File::create(path).unwrap())
    }

    /// Writes a string to the output.
    pub fn writeln(&mut self, s: &str) {
        writeln!(self, "{}", s).expect("should be able to write");
    }

    /// Converts the `ShellOutput` into a `Stdio`.
    pub fn as_stdio(&mut self) -> std::io::Result<Stdio> {
        match self {
            ShellOutput::File(ref mut file) => {
                let file_clone = file.try_clone()?;
                Ok(Stdio::from(file_clone))
            }
            ShellOutput::Stdout(_) => Ok(Stdio::inherit()),
            ShellOutput::Stderr(_) => Ok(Stdio::inherit()),
        }
    }
}

impl std::io::Write for ShellOutput {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            ShellOutput::Stdout(ref mut writer) => writer.write(buf),
            ShellOutput::Stderr(ref mut writer) => writer.write(buf),
            ShellOutput::File(ref mut writer) => writer.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            ShellOutput::Stdout(ref mut writer) => writer.flush(),
            ShellOutput::Stderr(ref mut writer) => writer.flush(),
            ShellOutput::File(ref mut writer) => writer.flush(),
        }
    }
}

pub struct Shell {
    pub stdout: ShellOutput,
    pub stderr: ShellOutput,

    stdin: std::io::Stdin,

    input_buffer: String,

    /// Registry of all registered commands (builtin and external).
    pub cmd_registry: CommandsRegistry,
}

impl Shell {
    /// Creates a new instance of the `Shell` struct.
    pub fn new() -> Self {
        Self {
            stdout: ShellOutput::stdout(),
            stderr: ShellOutput::stderr(),

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
            let eval_result = self.eval();
            self.handle_eval_result(eval_result);

            self.input_buffer.clear();
        }
    }

    /// Reads multiline input from the user. A line ending with an unescaped backslash (`\`)
    /// indicates that the command continues on the next line.
    fn read_multiline_input(&mut self) -> String {
        self.print_shell_header();
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

        let command = match self.cmd_registry.get_command(command_name) {
            Some(command) => command.clone(),
            None => {
                return Err(ShellError::CommandNotFound {
                    command_name: command_name.to_string(),
                });
            }
        };

        command.run(args, self)?;

        Ok(())
    }

    /// Prints the shell header. What did you expect?
    fn print_shell_header(&self) {
        println!(
            "\n\x1b[1;32m{}\x1b[0m", // bold, green
            std::env::current_dir().unwrap().display()
        );
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

    /// Handles the result of the `eval` function.
    fn handle_eval_result(&mut self, result: Result<(), ShellError>) {
        match result {
            Ok(_) => {}
            Err(err) => match err {
                ShellError::CommandNotFound { command_name } => {
                    self.stderr
                        .writeln(&format!("{}: command not found", command_name));

                    let mut levenshtein_threshold = 2;
                    if command_name.len() < 4 {
                        levenshtein_threshold = 1;
                    }

                    if let Some(closest_name) = Levenshtein::get_closest_with_threshold(
                        &command_name,
                        &self.cmd_registry.registered_names,
                        levenshtein_threshold,
                    ) {
                        self.stderr
                            .writeln(&format!("did you mean \"{}\"?", closest_name));
                    }
                }
                ShellError::EmptyInput => {
                    dprintln_err!("empty input error");
                }
                _ => {
                    self.stderr.writeln(&format!("{}", err));
                }
            },
        }
    }
}
