use ansi_term::Colour::{Cyan, Yellow};
use serde::Serialize;
use structopt::StructOpt;

use rover_client::operations::graph::delete::{self, GraphDeleteInput};

use crate::command::RoverOutput;
use crate::options::{GraphRefOpt, ProfileOpt};
use crate::utils::{self, client::StudioClientConfig};
use crate::Result;

#[derive(Debug, Serialize, StructOpt)]
pub struct Delete {
    #[structopt(flatten)]
    graph: GraphRefOpt,

    #[structopt(flatten)]
    profile: ProfileOpt,

    /// Skips the step where the command asks for user confirmation before
    /// deleting the graph.
    #[structopt(long)]
    confirm: bool,
}

impl Delete {
    pub fn run(&self, client_config: StudioClientConfig) -> Result<RoverOutput> {
        let client = client_config.get_authenticated_client(&self.profile.profile_name)?;
        let graph_ref = self.graph.graph_ref.to_string();

        eprintln!(
            "Deleting {} using credentials from the {} profile.",
            Cyan.normal().paint(&graph_ref),
            Yellow.normal().paint(&self.profile.profile_name)
        );

        if !self.confirm && !utils::confirm_delete()? {
            eprintln!("Delete cancelled by user");
            return Ok(RoverOutput::EmptySuccess);
        }

        delete::run(
            GraphDeleteInput {
                graph_ref: self.graph.graph_ref.clone(),
            },
            &client,
        )?;

        eprintln!("Successfully deleted {}.", Cyan.normal().paint(&graph_ref));
        Ok(RoverOutput::EmptySuccess)
    }
}
