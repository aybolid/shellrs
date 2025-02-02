use std::{
    fs::File,
    io::{self, Write},
    process::Stdio,
};

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
    Stdout(io::StdoutLock<'static>),
    Stderr(io::StderrLock<'static>),
    #[allow(dead_code)]
    File(File),
}

impl ShellOutput {
    pub fn stdout() -> Self {
        ShellOutput::Stdout(io::stdout().lock())
    }

    pub fn stderr() -> Self {
        ShellOutput::Stderr(io::stderr().lock())
    }

    #[allow(dead_code)]
    pub fn file(path: String) -> Self {
        ShellOutput::File(File::create(path).unwrap())
    }

    /// Writes a string to the output.
    pub fn writeln(&mut self, s: &str) {
        writeln!(self, "{}", s).expect("should be able to write");
    }

    /// Converts the `ShellOutput` into a `Stdio`.
    pub fn as_stdio(&mut self) -> io::Result<Stdio> {
        match self {
            ShellOutput::File(ref mut file) => Ok(Stdio::from(file.try_clone()?)),
            ShellOutput::Stdout(_) | ShellOutput::Stderr(_) => Ok(Stdio::inherit()),
        }
    }
}

impl io::Write for ShellOutput {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            ShellOutput::Stdout(ref mut writer) => writer.write(buf),
            ShellOutput::Stderr(ref mut writer) => writer.write(buf),
            ShellOutput::File(ref mut writer) => writer.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
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
    /// Registry of all registered commands (builtin and external).
    pub cmd_registry: CommandsRegistry,
    stdin: io::Stdin,
    input_buffer: String,
}

impl Shell {
    /// Creates a new instance of the `Shell` struct.
    pub fn new() -> Self {
        Self {
            stdout: ShellOutput::stdout(),
            stderr: ShellOutput::stderr(),
            cmd_registry: CommandsRegistry::default(),
            stdin: io::stdin(),
            input_buffer: String::new(),
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

    /// Reads multiline input from the user.
    ///
    /// A line ending with an unescaped backslash (`\`) indicates that the command continues on the next line.
    fn read_multiline_input(&mut self) -> String {
        self.print_shell_header();
        let mut complete_input = String::new();

        loop {
            print!("> ");
            self.stdout.flush().unwrap();

            let mut line = String::new();
            self.stdin.read_line(&mut line).unwrap();
            let line = line.trim_end_matches('\n');

            if line.ends_with('\\') && !line.ends_with("\\\\") {
                complete_input.push_str(line.trim_end_matches('\\'));
                complete_input.push(' ');
            } else {
                complete_input.push_str(line);
                break;
            }
        }

        complete_input
    }

    /// Evaluates the current input stored in `self.input_buffer`.
    fn eval(&mut self) -> Result<(), ShellError> {
        dprintln!("eval: {:?}", self.input_buffer);
        let tokens = self.parse_shell_input();
        dprintln!("parsed tokens: {:?}", tokens);

        if tokens.is_empty() {
            return Err(ShellError::EmptyInput);
        }

        let (command_tokens, stdout_redirect, stderr_redirect) =
            self.process_redirections(tokens)?;

        if command_tokens.is_empty() {
            return Err(ShellError::EmptyInput);
        }

        let command_name = &command_tokens[0];
        let args: Vec<&str> = command_tokens[1..].iter().map(|s| s.as_str()).collect();

        dprintln!("command name: {}", command_name);
        dprintln!("args: {:?}", args);
        dprintln!("stdout redirection: {:?}", stdout_redirect);
        dprintln!("stderr redirection: {:?}", stderr_redirect);

        let command = self
            .cmd_registry
            .get_command(command_name)
            .cloned()
            .ok_or_else(|| ShellError::CommandNotFound {
                command_name: command_name.clone(),
            })?;

        // swap out stdout and stderr if necessary.
        let original_stdout = stdout_redirect
            .map(|file| std::mem::replace(&mut self.stdout, ShellOutput::file(file)));
        let original_stderr = stderr_redirect
            .map(|file| std::mem::replace(&mut self.stderr, ShellOutput::file(file)));

        // run the command.
        let result = command.run(args, self);

        // restore original outputs.
        if let Some(stdout) = original_stdout {
            self.stdout = stdout;
        }
        if let Some(stderr) = original_stderr {
            self.stderr = stderr;
        }

        result
    }

    /// Processes tokens to separate redirection tokens from command tokens.
    ///
    /// Returns a tuple of:
    /// - command tokens,
    /// - optional stdout redirection file,
    /// - optional stderr redirection file.
    fn process_redirections(
        &self,
        tokens: Vec<String>,
    ) -> Result<(Vec<String>, Option<String>, Option<String>), ShellError> {
        let mut command_tokens = Vec::new();
        let mut stdout_redirect = None;
        let mut stderr_redirect = None;

        let mut iter = tokens.into_iter();
        while let Some(token) = iter.next() {
            match token.as_str() {
                "&>" => {
                    let file = iter.next().ok_or_else(|| {
                        ShellError::CommandExecutionFail(
                            "no file specified for output redirection".to_string(),
                        )
                    })?;
                    stdout_redirect = Some(file.clone());
                    stderr_redirect = Some(file);
                }
                ">" | "1>" => {
                    stdout_redirect = Some(iter.next().ok_or_else(|| {
                        ShellError::CommandExecutionFail(
                            "no file specified for output redirection".to_string(),
                        )
                    })?);
                }
                "2>" => {
                    stderr_redirect = Some(iter.next().ok_or_else(|| {
                        ShellError::CommandExecutionFail(
                            "no file specified for error output redirection".to_string(),
                        )
                    })?);
                }
                _ => command_tokens.push(token),
            }
        }

        Ok((command_tokens, stdout_redirect, stderr_redirect))
    }

    /// Parses the shell input into tokens.
    ///
    /// Supports whitespace separation, quoting (single and double), and escaping.
    fn parse_shell_input(&self) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();

        let mut in_single_quote = false;
        let mut in_double_quote = false;

        let mut chars = self.input_buffer.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '\'' if !in_double_quote => {
                    in_single_quote = !in_single_quote;
                }
                '"' if !in_single_quote => {
                    in_double_quote = !in_double_quote;
                }
                '\\' => {
                    if let Some(escaped_char) = chars.next() {
                        current.push(escaped_char);
                    }
                }
                ' ' | '\t' if !in_single_quote && !in_double_quote => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    // Skip any additional consecutive whitespace.
                    while let Some(&next_char) = chars.peek() {
                        if next_char == ' ' || next_char == '\t' {
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }
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

    /// Prints the shell header (current working directory in bold green).
    fn print_shell_header(&self) {
        if let Ok(path) = std::env::current_dir() {
            println!("\n\x1b[1;32m{}\x1b[0m", path.display());
        }
    }

    /// Handles the result of evaluating a command.
    fn handle_eval_result(&mut self, result: Result<(), ShellError>) {
        if let Err(err) = result {
            match err {
                ShellError::CommandNotFound { command_name } => {
                    self.stderr
                        .writeln(&format!("{}: command not found", command_name));

                    let levenshtein_threshold = if command_name.len() < 4 { 1 } else { 2 };

                    if let Some(closest) = Levenshtein::get_closest_with_threshold(
                        &command_name,
                        &self.cmd_registry.registered_names,
                        levenshtein_threshold,
                    ) {
                        self.stderr
                            .writeln(&format!("did you mean \"{}\"?", closest));
                    }
                }
                ShellError::EmptyInput => {
                    dprintln_err!("empty input error");
                }
                _ => {
                    self.stderr.writeln(&format!("{}", err));
                }
            }
        }
    }
}
