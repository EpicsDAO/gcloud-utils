use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use console::style;
use gcloud_utils::cli::{Cli, Commands, GcpConfig, IamCommands, ComputeCommands, InitCommands, GhCommands, RunCommands};
use gcloud_utils::gh::*;
use gcloud_utils::iam::*;
use gcloud_utils::compute::*;
use gcloud_utils::init::*;
use gcloud_utils::run::*;
use std::path::Path;
use gcloud_utils::constants::{PAPER_EMOJI};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let file_name = "gcp_config.json";
    let file_exist = Path::new(file_name).exists();
    if file_exist == false {
        process_init_gcp_config().await;
    }
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);
    let gcp: GcpConfig = serde_json::from_reader(reader).unwrap();
    match cli.command {
        Commands::Iam(iam) => {
            let iam_cmd = iam.command.unwrap_or(IamCommands::Help);
            match iam_cmd {
                IamCommands::Setup => {
                    process_create_service_account(gcp.project_id.as_str(), gcp.service_name.as_str()).await;
                    process_create_service_account_key(
                        gcp.project_id.as_str(),
                        gcp.service_name.as_str(),
                    )
                    .await;
                    process_add_roles(gcp.project_id.as_str(), gcp.service_name.as_str()).await;
                    process_enable_permissions(gcp.project_id.as_str()).await;
                }
                _ => {
                    println!("{}{}", PAPER_EMOJI, style("To see example;\n\n $gcu iam --help").white().bold());
                }
            }
        },
        Commands::Run(run) =>  {
            let run_cmd = run.command.unwrap_or(RunCommands::Help);
            match run_cmd {
                RunCommands::Deploy => {
                    process_gcloud_build(&gcp.project_id, &gcp.service_name).await;
                    process_deploy(&gcp.project_id, &gcp.service_name).await;
                }
                _ => {
                    println!("{}{}", PAPER_EMOJI, style("To see example;\n\n $gcu run --help").white().bold());
                }
            }
        },
        Commands::Gh(gh) =>  {
            let gh_cmd = gh.command.unwrap_or(GhCommands::Help);
            match gh_cmd {
                GhCommands::AddEnv => {
                    process_setup_secret().await;
                }
                _ => {
                    println!("{}{}", PAPER_EMOJI, style("To see example;\n\n $gcu gh --help").white().bold());
                }
            }
        },
        Commands::Init(init) => {
            let init_cmd = init.command.unwrap_or(InitCommands::Help);
            match init_cmd {
                InitCommands::Config => {
                    process_init_gcp_config().await;
                }
                _ => {
                    println!("{}{}", PAPER_EMOJI, style("To see example;\n\n $gcu init --help").white().bold());
                }
            }
        },
        Commands::Compute(compute) => {
            let compute_cmd = compute.command.unwrap_or(ComputeCommands::Help);
            match compute_cmd {
                ComputeCommands::CreateNat => {
                    process_create_network(&gcp.project_id, &gcp.service_name).await;
                    process_create_firewall_tcp(&gcp.project_id, &gcp.service_name).await;
                    process_create_firewall_ssh(&gcp.project_id, &gcp.service_name).await;
                    process_create_subnet(&gcp.project_id, &gcp.service_name, &gcp.region).await;
                    process_create_connector(&gcp.project_id, &gcp.service_name,  &gcp.region).await;
                    process_create_router(&gcp.project_id, &gcp.service_name, &gcp.region).await;
                    process_create_external_ip(&gcp.project_id, &gcp.service_name, &gcp.region).await;
                    process_create_nat(&gcp.project_id, &gcp.service_name, &gcp.region).await;
                }
                _ => {
                    println!("{}{}", PAPER_EMOJI, style("To see example;\n\n $gcu compute --help").white().bold());
                }
            }
        },
    }
}