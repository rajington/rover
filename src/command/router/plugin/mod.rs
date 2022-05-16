mod new;

use crate::command::RoverOutput;
use crate::Result;
use serde::Serialize;
use structopt::StructOpt;

#[derive(Debug, Serialize, StructOpt)]
pub struct Plugin {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, Serialize, StructOpt)]
pub enum Command {
    New(new::New),
}

impl Plugin {
    pub fn run(&self) -> Result<RoverOutput> {
        match &self.command {
            Command::New(command) => command.run(),
        }
    }
}
