mod download;
mod plugin;

use crate::command::RoverOutput;
use crate::Result;
use serde::Serialize;
use structopt::StructOpt;

#[derive(Debug, Serialize, StructOpt)]
pub struct Router {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, Serialize, StructOpt)]
pub enum Command {
    Plugin(plugin::Plugin),
    Download(download::Download),
}

impl Router {
    pub fn run(&self) -> Result<RoverOutput> {
        match &self.command {
            Command::Plugin(command) => command.run(),
            Command::Download(command) => command.run(),
        }
    }
}
