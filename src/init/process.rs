use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Write};
use std::io;

#[derive(Serialize, Deserialize, Debug)]
pub struct GcpConfig {
  pub project_id: String,
  pub service_name: String,
  pub region: String,
}


pub async fn process_init_gcp_config() {
  println!("Please input your GCP project_id:");
  let mut project_id = String::new();
  io::stdin()
      .read_line(&mut project_id)
      .expect("Failed to read line");
  let project_id: String = project_id
    .trim()
    .parse()
    .expect("Please input your GCP project_id:");

  println!("Please input your GCP service_name:");
  let mut service_name = String::new();
  io::stdin()
      .read_line(&mut service_name)
      .expect("Failed to read line");
  let service_name: String = service_name
    .trim()
    .parse()
    .expect("Please input your GCP service_name:");

  println!("Please input your GCP region:");
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
      println!("Generated `./gcp_config.json` file!")
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