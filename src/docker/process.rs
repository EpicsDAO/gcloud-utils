use tokio::process::Command;

pub async fn process_docker_build(project_id: &str, service_name: &str) {
  let gcr_url = String::from("eu.gcr.io/") + project_id + "/" + service_name;
  let output = Command::new("docker")
    .args(
      &[
        "build",
        ".",
        "-t",
        &gcr_url,
        ])
    .output()
    .await;
  println!("output = {:?}", output);
}