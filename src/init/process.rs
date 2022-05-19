use console::style;
use serde::{Deserialize, Serialize};
use std::env::current_exe;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct GcpConfig {
  pub project_id: String,
  pub service_name: String,
  pub region: String,
}

pub async fn process_init_gcp_config() {
  println!(
    "📝 {}",
    style("Please input your GCP project_id:").white().bold()
  );
  let mut project_id = String::new();
  io::stdin()
    .read_line(&mut project_id)
    .expect("Failed to read line");
  let project_id: String = project_id
    .trim()
    .parse()
    .expect("Please input your GCP project_id:");

  println!(
    "📝 {}",
    style("Please input your GCP service_name:").white().bold()
  );
  let mut service_name = String::new();
  io::stdin()
    .read_line(&mut service_name)
    .expect("Failed to read line");
  let service_name: String = service_name
    .trim()
    .parse()
    .expect("Please input your GCP service_name:");

  println!(
    "📝 {}",
    style("Please input your GCP region:").white().bold()
  );
  let mut region = String::new();
  io::stdin()
    .read_line(&mut region)
    .expect("Failed to read line");
  let region: String = region
    .trim()
    .parse()
    .expect("Please input your GCP region:");

  let json_struct = build_gcp_config(project_id, service_name, region).await;
  let result = write_gcp_config(json_struct).await;
  match result {
    Ok(..) => {
      println!("✅ {}", style("Successfully Generated!").green().bold());
      println!("🗃️ {}", style("File Path: ./gcp_config.json").white().bold());
    }
    Err(err) => {
      println!("Failed to Write: {}", err)
    }
  }
}

async fn write_gcp_config(json_struct: GcpConfig) -> std::io::Result<()> {
  let serialized: String = serde_json::to_string_pretty(&json_struct).unwrap();
  let mut file = File::create("gcp_config.json")?;
  file.write_all(serialized.as_bytes())?;
  Ok(())
}

async fn build_gcp_config(project_id: String, service_name: String, region: String) -> GcpConfig {
  GcpConfig {
    project_id,
    service_name,
    region,
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiWorkflow {
  pub name: String,
  pub on: Push,
  pub jobs: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Push {
  pub push: Branch,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Branch {
  pub branches: String,
}

pub async fn build_api_workflow(nat: bool) -> std::io::Result<()> {
  let current_exe = std::env::current_exe().unwrap();
  let file_path = String::from(current_exe.to_string_lossy());
  let workflow_dir = file_path.to_string() + ".github/workflow";
  fs::create_dir_all(workflow_dir).unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });
  let workflow_template = match nat {
    true => file_path.to_string() + ".github/workflow/nat.yml",
    false => file_path.to_string() + ".github/workflow/default.yml",
  };
  let workflow_yml = ".github/workflows/epic_service.yml";
  let file_exist = Path::new(workflow_yml).exists();
  match file_exist {
    true => {
      panic!("File already exist!")
    }
    false => {
      let _ = fs::copy(workflow_template, workflow_yml);
      println!("✅ {}", style("Successfully Generated!").green().bold());
    }
  }
  println!("{:?}", std::env::current_exe());
  Ok(())
}
