use crate::{
    app::{Shell, ShellError},
    commands::Command,
    dprintln,
};

#[derive(Debug)]
pub struct CdCommand;

impl Command for CdCommand {
    fn run(&self, args: Vec<&str>, _: &mut Shell) -> Result<(), ShellError> {
        let target_dir = match args.get(0) {
            Some(arg) => arg.to_string(),
            None => std::env::var("HOME").unwrap_or_else(|_| "/".to_string()),
        };

        let target_dir = std::path::Path::new(&target_dir);
        dprintln!("changing directory to {:?}", target_dir);

        std::env::set_current_dir(target_dir)
            .map_err(|err| ShellError::CommandExecutionFail(err.to_string()))?;

        Ok(())
    }

    fn get_name(&self) -> String {
        "cd".to_string()
    }

    fn get_help_message(&self, _: &mut Shell) -> Result<String, ShellError> {
        let mut help_message = String::new();

        help_message.push_str(format!("usage: {} <directory>\n", self.get_name()).as_str());
        help_message
            .push_str("changes the current working directory to the specified directory.\n");
        help_message
            .push_str("if no directory is specified, the HOME environment variable is used.");

        Ok(help_message)
    }
}
