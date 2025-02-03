use thiserror::Error;

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
    /// The shell input could not be parsed.
    /// The message will be formatted as an error message (red color).
    #[error("\x1b[31m{0}\x1b[0m")]
    ParsingFail(String),
}
