use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use tokio::process::Command;
use console::style;
use gcloud_utils::cli::{Cli, Commands, GcpConfig};
use gcloud_utils::gh::*;
use gcloud_utils::iam::*;
use gcloud_utils::run::*;
use gcloud_utils::init::*;
use std::path::Path;
use gcloud_utils::constants::{COMPLETE_EMOJI, ERROR_EMOJI};

#[tokio::main]
async fn main() {
    println!("{}{}", COMPLETE_EMOJI, style("Command successful.").green().bold());
    let cli = Cli::parse();
    let file_name = "gcp_config.json";
    let file_exist = Path::new(file_name).exists();
    if file_exist == false {
        process_init_gcp_config().await;
    }
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);
    let gcp: GcpConfig = serde_json::from_reader(reader).unwrap();
    config_set(&gcp.project_id).await;
    match cli.command {
        Commands::Iam { action } => match &*action {
            "setup" => {
                process_create_service_account(gcp.service_name.as_str()).await;
                process_create_service_account_key(
                    gcp.service_name.as_str(),
                    gcp.project_id.as_str(),
                )
                .await;
                process_add_roles(gcp.service_name.as_str(), gcp.project_id.as_str()).await;
            }
            _ => println!("no command!"),
        },
        Commands::Run { action } => match &*action {
            "deploy" => {
                process_build(&gcp.service_name, &gcp.project_id).await;
                process_deploy(&gcp.service_name, &gcp.project_id).await;
            }
            _ => println!("no command!"),
        },
        Commands::Gh { action } => match &*action {
            "add-env" => {
                process_setup_secret().await;
            }
            _ => println!("no command!"),
        },
        Commands::Init { action } => match &*action {
            "config" => {
                if file_exist != false {
                    process_init_gcp_config().await;
                }
            }
            _ => println!("no command!"),
        }
    }
}

async fn config_set(project_id: &str) {
    let _output = Command::new("gcloud")
        .args(&["config", "set", "project", project_id])
        .output()
        .await;
    // println!("output = {:?}", output);
}
