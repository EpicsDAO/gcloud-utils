use tokio::process::Command;
use console::style;
use std::io;
use regex::Regex;
use std::str;

fn regex(re_str: &str) -> Regex {
  Regex::new(re_str).unwrap()
}

pub async fn process_create_sql(project_id: &str, service_name: &str, region: &str) {
  println!(
    "📝 {}",
    style("Please input your DB Root Password:").white().bold()
  );
  let mut db_password = String::new();
  io::stdin()
    .read_line(&mut db_password)
    .expect("Failed to read line");
  let db_password: String = db_password
    .trim()
    .parse()
    .expect("Please input DB Root Password:");
  let zone = String::from(region) + "-b";
  println!(
    "⏰ {}",
    style("Creating Cloud SQL ...\nThis process takes 5 to 10 min.").white().bold()
  );
  let instance_name = String::from(service_name) + "-db";
  let db_version = String::from("--database-version=POSTGRES_14");
  let output = Command::new("gcloud")
    .args(&[
      "sql",
      "instances",
      "create",
      &instance_name,
      &db_version,
      "--cpu=1",
      "--memory=4096MB",
      "--zone",
      &zone,
      "--root-password",
      &db_password,
      "--database-flags",
      "cloudsql.iam_authentication=on",
      "--project",
      project_id
    ])
    .output()
    .await;
  match &output {
    Ok(val) => {
      let err = str::from_utf8(&val.stderr);
      let rt = regex("ERROR:");
      match rt.is_match(err.unwrap()) {
        true => {
            panic!("{:?}", err.unwrap())
        }
        false => {
          println!(
              "✅ {}",
              style("Successfully created Cloud SQL!").white().bold()
          );
        }
      }
    },
    Err(err) => println!("error = {:?}", err)
  }
}

async fn region_to_timezone(region: &str) -> &str {
  let asia = regex("asia");
  let eu = regex("europe");
  let zone = if asia.is_match(region)  {
    "Asia/Tokyo"
  } else if eu.is_match(region) {
    "Europe/Amsterdam"
  } else {
    "America/Los_Angeles"
  };
  zone
}