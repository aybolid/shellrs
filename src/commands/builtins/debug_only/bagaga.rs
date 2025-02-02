use rand::Rng;

use crate::{
    app::{Shell, ShellError},
    commands::Command,
};

const RESPONSES: [&str; 4] = [
    "what is, is",
    "i know what it is! it is barbipines sinken blue.",
    "PNN.TPN.PNH",
    "buckets ding-ding",
];

#[derive(Debug)]
pub struct BagagaCommand;

impl Command for BagagaCommand {
    fn run(&self, _: Vec<&str>, shell: &mut Shell) -> Result<(), ShellError> {
        shell
            .stdout
            .writeln(RESPONSES[rand::rng().random_range(0..RESPONSES.len())]);
        Ok(())
    }

    fn get_name(&self) -> String {
        "bagaga".to_string()
    }

    fn get_help_message(&self, _: &mut Shell) -> Result<String, ShellError> {
        let mut help_message = String::new();

        help_message.push_str("bagaga wont help you.");

        Ok(help_message)
    }
}
