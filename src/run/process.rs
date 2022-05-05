use tokio::process::Command;

pub async fn process_build(service_name: &str, project_id: &str) {
  let gcr_url = String::from("gcr.io/") + project_id + "/" + service_name;
  println!("{}", gcr_url);
  let output = Command::new("gcloud")
    .args(&[
      "builds",
      "submit",
      "--tag",
      project_id,
      &gcr_url,
      "--timeout=80000",
    ])
    .output()
    .await;
  println!("output = {:?}", output);
}

pub async fn process_deploy(service_name: &str, project_id: &str) {
  let img_url = String::from("gcr.io/") + project_id + "/" + service_name;
  println!("{}", &img_url);
  let output = Command::new("gcloud")
    .args(&["run", "deploy", "--image", &img_url])
    .output()
    .await;
  println!("output = {:?}", output);
}
