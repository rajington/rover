use serde::Serialize;
use structopt::StructOpt;

use crate::command::RoverOutput;
use crate::Result;

#[derive(Debug, Serialize, StructOpt)]
pub struct Download {}

impl Download {
    pub fn run(&self) -> Result<RoverOutput> {
        Ok(RoverOutput::EmptySuccess)
    }
}
