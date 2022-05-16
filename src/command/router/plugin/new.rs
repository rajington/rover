use cargo_scaffold::{Opts, ScaffoldDescription};
use serde::Serialize;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::command::RoverOutput;
use crate::Result;

#[derive(Debug, Serialize, StructOpt)]
pub struct New {}

impl New {
    pub fn run(&self) -> Result<RoverOutput> {
        let opts = Opts {
            append: false,
            force: false,
            passphrase_needed: false,
            project_name: String::from("testlib").into(),
            template_path: PathBuf::from("https://github.com/Cosmian/mpc_rust_template.git"),
            target_dir: None,
            template_commit: None,
        };
        ScaffoldDescription::new(opts)?.scaffold()?;
        Ok(RoverOutput::EmptySuccess)
    }
}
