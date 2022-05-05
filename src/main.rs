use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use tokio::process::Command;

use std::fs::File;
use std::io::BufReader;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Iam { action: String },
    Greet { name: String },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let file_name = "gcp_config.json";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let gcp: GcpConfig = serde_json::from_reader(reader).unwrap();
    match cli.command {
        Commands::Iam { action } => {
            if action == "create".to_string() {
                create_service_account(gcp.service_name.as_str()).await;
            } else if action == "create-key" {
                create_service_account_key(gcp.service_name.as_str(), gcp.project_id.as_str())
                    .await;
            } else if action == "roles" {
                add_roles().await;
            } else {
                println!("no command with '{}'", action);
            }
        }
        Commands::Greet { name } => {
            println!("hello, {}", name);
        }
    }
}

async fn create_service_account(service_name: &str) {
    let description = String::from("--description='") + service_name + " Service Account'";
    let display_name = String::from("--display-name=") + service_name;
    let output = Command::new("gcloud")
        .args(&[
            "iam",
            "service-accounts",
            "create",
            service_name,
            description.as_str(),
            display_name.as_str(),
        ])
        .output()
        .await;
    println!("output = {:?}", output);
}

async fn create_service_account_key(service_name: &str, project_id: &str) {
    let service_account =
        String::from(service_name) + "@" + project_id + ".iam.gserviceaccount.com";
    let output = Command::new("gcloud")
        .args(&[
            "iam",
            "service-accounts",
            "keys",
            "create",
            "./keyfile.json",
            "--iam-account",
            service_account.as_str(),
        ])
        .output()
        .await;
    println!("output = {:?}", output);
}

async fn add_roles() {
    let roles = [
        "roles/cloudsql.editor",
        "roles/containerregistry.ServiceAgent",
        "roles/pubsub.editor",
        "roles/datastore.user",
        "roles/iam.serviceAccountUser",
        "roles/run.admin",
        "roles/storage.admin",
        "roles/storage.objectAdmin",
        "roles/cloudscheduler.admin",
        "roles/appengine.appCreator",
        "roles/logging.admin",
        "roles/cloudtranslate.admin",
    ];
    for role in roles {
        add_service_account_role(role).await;
    }
}

async fn add_service_account_role(role_arg: &str) {
    let service_name = "epic-gcu";
    let project_id = "degitana-app";
    let member = String::from("--member=serviceAccount:")
        + service_name
        + "@"
        + project_id
        + ".iam.gserviceaccount.com";
    let role = String::from("--role=") + role_arg;
    let output = Command::new("gcloud")
        .args(&[
            "projects",
            "add-iam-policy-binding",
            project_id,
            member.as_str(),
            role.as_str(),
        ])
        .output()
        .await;
    println!("output = {:?}", output);
}

#[derive(Serialize, Deserialize, Debug)]
struct GcpConfig {
    project_id: String,
    service_name: String,
    region: String,
}
