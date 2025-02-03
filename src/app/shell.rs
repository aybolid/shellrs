use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;

use libc::{tcgetattr, tcsetattr, termios, ECHO, ICANON, TCSANOW};

use levenshtein::Levenshtein;

use super::{ShellError, ShellOutput};
use crate::{commands::CommandsRegistry, dprintln, dprintln_err};

pub struct Shell {
    /// The standard output of the shell.
    pub stdout: ShellOutput,
    /// The standard error output of the shell.
    pub stderr: ShellOutput,
    /// Registry of all registered commands (builtin and external).
    pub cmd_registry: CommandsRegistry,
    /// Buffer for storing user input.
    input_buffer: String,
    /// The TTY file handle used for reading raw input.
    tty: File,
}

impl Shell {
    /// Creates a new instance of the `Shell` struct.
    pub fn new() -> Self {
        let tty = File::open("/dev/tty").expect("failed to open /dev/tty");

        Self {
            stdout: ShellOutput::stdout(),
            stderr: ShellOutput::stderr(),
            cmd_registry: CommandsRegistry::default(),
            input_buffer: String::new(),
            tty,
        }
    }

    /// Runs the shell REPL (Read-Eval-Print-Loop).
    pub fn run_repl(&mut self) {
        dprintln!("starting repl");
        loop {
            self.input_buffer = self.handle_input();
            if let Err(err) = self.eval() {
                self.handle_eval_error(err);
            }
            self.input_buffer.clear();
        }
    }

    /// Handles user input by putting the TTY into raw mode, reading key-by-key,
    /// and then restoring the original terminal settings.
    fn handle_input(&mut self) -> String {
        let fd = self.tty.as_raw_fd();

        // save the original terminal settings.
        let original_termios = Self::get_termios(fd).unwrap();

        // set terminal to raw mode (disable canonical mode and echo).
        let raw_termios = Self::disable_canonical_echo(original_termios);
        Self::set_termios(fd, &raw_termios).unwrap();

        let prompt = "> ";
        let mut buffer = String::new();
        let mut cursor_pos = 0;

        self.print_shell_header();
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let redraw_line = |buffer: &str, cursor_pos: usize| {
            // \r returns to the beginning of the line; \x1b[K clears the line from the cursor onward.
            print!("\r{}{}\x1b[K", prompt, buffer);
            // move the cursor to the correct position.
            let cursor_col = prompt.len() + cursor_pos + 1;
            print!("\r\x1b[{}G", cursor_col);
            io::stdout().flush().unwrap();
        };

        loop {
            let mut byte = [0u8; 1];
            if self.tty.read(&mut byte).unwrap() == 0 {
                break;
            }
            let b = byte[0];

            match b {
                b'\n' | b'\r' => {
                    println!();
                    break;
                }
                0x1B => {
                    // possibly an escape sequence.
                    let mut seq = [0u8; 2];
                    if self.tty.read(&mut seq).unwrap() < 2 {
                        continue;
                    }
                    if seq[0] == b'[' {
                        match seq[1] {
                            b'D' => {
                                // left arrow: move cursor left.
                                if cursor_pos > 0 {
                                    cursor_pos -= 1;
                                }
                            }
                            b'C' => {
                                // right arrow: move cursor right.
                                if cursor_pos < buffer.len() {
                                    cursor_pos += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                127 | 8 => {
                    // handle backspace.
                    if cursor_pos > 0 {
                        buffer.remove(cursor_pos - 1);
                        cursor_pos -= 1;
                    }
                }
                0x04 => break, // Ctrl-D (EOF).
                _ if b.is_ascii() && !b.is_ascii_control() => {
                    // insert printable character.
                    let ch = b as char;
                    buffer.insert(cursor_pos, ch);
                    cursor_pos += 1;
                }
                _ => {}
            }

            redraw_line(&buffer, cursor_pos);
        }

        // restore the original terminal settings.
        Self::set_termios(fd, &original_termios).expect("failed to restore terminal settings");

        buffer
    }

    /// Helper function to get terminal attributes.
    fn get_termios(fd: i32) -> io::Result<termios> {
        unsafe {
            let mut term = std::mem::zeroed::<termios>();
            if tcgetattr(fd, &mut term) != 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(term)
            }
        }
    }

    /// Helper function to set terminal attributes.
    fn set_termios(fd: i32, term: &termios) -> io::Result<()> {
        unsafe {
            if tcsetattr(fd, TCSANOW, term) != 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(())
            }
        }
    }

    /// Returns a modified termios with canonical mode and echo disabled.
    fn disable_canonical_echo(mut term: termios) -> termios {
        term.c_lflag &= !(ICANON | ECHO);
        term
    }

    /// Evaluates the current input stored in `self.input_buffer`.
    fn eval(&mut self) -> Result<(), ShellError> {
        dprintln!("eval input: {:?}", self.input_buffer);
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
        let args: Vec<&str> = command_tokens[1..].iter().map(String::as_str).collect();

        dprintln!("cmd name: {}", command_name);
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

        // swap out stdout and stderr if redirection is specified.
        let original_stdout = stdout_redirect
            .map(|file| std::mem::replace(&mut self.stdout, ShellOutput::file(file)));
        let original_stderr = stderr_redirect
            .map(|file| std::mem::replace(&mut self.stderr, ShellOutput::file(file)));

        // execute the command.
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
                        ShellError::ParsingFail(
                            "no file specified for output redirection".to_string(),
                        )
                    })?;
                    stdout_redirect = Some(file.clone());
                    stderr_redirect = Some(file);
                }
                ">" | "1>" => {
                    stdout_redirect = Some(iter.next().ok_or_else(|| {
                        ShellError::ParsingFail(
                            "no file specified for output redirection".to_string(),
                        )
                    })?);
                }
                "2>" => {
                    stderr_redirect = Some(iter.next().ok_or_else(|| {
                        ShellError::ParsingFail(
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
    fn parse_shell_input(&self) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();

        let mut in_single_quote = false;
        let mut in_double_quote = false;
        let mut chars = self.input_buffer.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '\'' if !in_double_quote => in_single_quote = !in_single_quote,
                '"' if !in_single_quote => in_double_quote = !in_double_quote,
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
                    // skip additional whitespaces
                    while let Some(&next_char) = chars.peek() {
                        if next_char == ' ' || next_char == '\t' {
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }
                _ => current.push(c),
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
            println!("  \x1b[1;32m{}\x1b[0m", path.display());
        }
    }

    /// Handles the result of evaluating a command.
    fn handle_eval_error(&mut self, error: ShellError) {
        match error {
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
            err => {
                self.stderr.writeln(&err.to_string());
            }
        }
    }
}
