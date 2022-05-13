use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GcpConfig {
  pub project_id: String,
  pub service_name: String,
  pub region: String,
}

#[derive(Parser)]
pub struct Cli {
  #[clap(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  Iam { action: String },
  Run { action: String },
  Gh { action: String },
  Init { action: String },
  Compute { action: String },
}
