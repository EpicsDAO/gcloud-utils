use tokio::process::Command;

pub async fn process_docker_build(project_id: &str, service_name: &str) {
  let gcr_url = String::from("eu.gcr.io/") + project_id + "/" + service_name;
  let output = Command::new("docker")
    .args(&["build", ".", "-t", &gcr_url])
    .output()
    .await;
  println!("output = {:?}", output);
}

pub async fn process_docker_push(project_id: &str, service_name: &str) {
  let gcr_url = String::from("eu.gcr.io/") + project_id + "/" + service_name;
  let output = Command::new("docker")
    .args(&["push", &gcr_url])
    .output()
    .await;
  println!("output = {:?}", output);
}

pub async fn process_docker_psql() {
  let output = Command::new("docker")
    .args(&[
      "run",
      "--rm",
      "-d",
      "--name",
      "epics-psql",
      "-p",
      "5432:5432",
      "-v",
      "postgres-tmp:/home/postgresql/data",
      "-e",
      "POSTGRES_USER=postgres",
      "-e",
      "POSTGRES_PASSWORD=postgres",
      "-e",
      "POSTGRES_DB=epics_test",
      "postgres:14.3-alpine",
    ])
    .output()
    .await;
  println!("output = {:?}", output);
}
