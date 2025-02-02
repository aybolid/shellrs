use rand::Rng;

use crate::{
    app::{ShellError, ShellOutput},
    commands::{Command, CommandsRegistry},
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
    fn run(
        &self,
        out: &mut ShellOutput,
        _: Vec<&str>,
        _: &CommandsRegistry,
    ) -> Result<(), ShellError> {
        out.writeln(RESPONSES[rand::rng().random_range(0..RESPONSES.len())]);
        Ok(())
    }

    fn get_name(&self) -> String {
        "bagaga".to_string()
    }

    fn get_help_message(
        &self,
        _: &mut ShellOutput,
        _: &CommandsRegistry,
    ) -> Result<String, ShellError> {
        let mut help_message = String::new();

        help_message.push_str("bagaga wont help you.");

        Ok(help_message)
    }
}
