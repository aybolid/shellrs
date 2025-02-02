use rand::Rng;

use crate::{
    app::ShellError,
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
    fn run(&self, _: Vec<&str>, _: &CommandsRegistry) -> Result<(), ShellError> {
        println!(
            "{}",
            RESPONSES[rand::rng().random_range(0..RESPONSES.len())]
        );
        Ok(())
    }

    fn get_name(&self) -> String {
        "bagaga".to_string()
    }

    fn get_type_message(&self) -> String {
        format!(
            "{} is a... hm. type data seems to be encrypted",
            self.get_name()
        )
    }
}
