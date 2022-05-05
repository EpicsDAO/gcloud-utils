use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use tokio::process::Command;

use gcloud_utils::cli::{Cli, Commands, GcpConfig};
use gcloud_utils::iam::*;
use gcloud_utils::run::process_build;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let file_name = "gcp_config.json";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let gcp: GcpConfig = serde_json::from_reader(reader).unwrap();
    config_set(&gcp.project_id).await;
    match cli.command {
        Commands::Iam { action } => {
            if action == "setup" {
                process_create_service_account(gcp.service_name.as_str()).await;
                process_create_service_account_key(
                    gcp.service_name.as_str(),
                    gcp.project_id.as_str(),
                )
                .await;
                process_add_roles(gcp.service_name.as_str(), gcp.project_id.as_str()).await;
            } else {
                println!("no command!");
            }
        }
        Commands::Run { name } => {
            process_build(&gcp.service_name, &gcp.project_id).await;
            println!("hello, {}", name);
        }
    }
}

async fn config_set(project_id: &str) {
    let output = Command::new("gcloud")
        .args(&["config", "set", "project", project_id])
        .output()
        .await;
    println!("output = {:?}", output);
}
