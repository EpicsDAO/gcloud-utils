use tokio::process::Command;

pub async fn process_create_service_account(service_name: &str) {
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

pub async fn process_create_service_account_key(service_name: &str, project_id: &str) {
  let service_account = String::from(service_name) + "@" + project_id + ".iam.gserviceaccount.com";
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

pub async fn process_add_roles(service_name: &str, project_id: &str) {
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
    process_add_service_account_role(service_name, project_id, role).await;
  }
}

pub async fn process_add_service_account_role(
  service_name: &str,
  project_id: &str,
  role_arg: &str,
) {
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
