use tokio::process::Command;

pub async fn process_create_network(service_name: &str) {
  let output = Command::new("gcloud")
    .args(&["compute", "networks", "create", service_name])
    .output()
    .await;
  println!("output = {:?}", output);
}

pub async fn process_create_firewall_tcp(service_name: &str) {
  let output = Command::new("gcloud")
    .args(&[
      "compute",
      "firewall-rules",
      "create",
      "--network",
      service_name,
      "--allow",
      "tcp,udp,icmp",
      "--source-ranges",
      "10.124.0.0/28",
    ])
    .output()
    .await;
  println!("output = {:?}", output);
}

pub async fn process_create_firewall_ssh(service_name: &str) {
  let firewall = String::from(service_name) + "-ssh";
  let output = Command::new("gcloud")
    .args(&[
      "compute",
      "firewall-rules",
      "create",
      &firewall,
      "--network",
      service_name,
      "--allow",
      "tcp:22,tcp:3389,icmp",
    ])
    .output()
    .await;
  println!("output = {:?}", output);
}

pub async fn process_create_subnet(service_name: &str, region: &str) {
  let subnet = String::from(service_name) + "-subnet";
  let network = String::from("--network=") + service_name;
  let region_str = String::from("--region=") + region;
  let output = Command::new("gcloud")
    .args(&[
      "compute",
      "networks",
      "subnets",
      "create",
      &subnet,
      "--range=10.124.0.0/28",
      &network,
      &region_str,
    ])
    .output()
    .await;
  println!("output = {:?}", output);
}

pub async fn process_create_connector(service_name: &str, project_id: &str, region: &str) {
  let subnet = String::from("--subnet=") + service_name + "-subnet";
  let subnet_project = String::from("--subnet-project=") + project_id;
  let region_str = String::from("--region=") + region;
  let output = Command::new("gcloud")
    .args(&[
      "compute",
      "networks",
      "vpc-access",
      "connectors",
      "create",
      service_name,
      &subnet,
      &subnet_project,
      &region_str,
    ])
    .output()
    .await;
  println!("output = {:?}", output);
}

pub async fn process_create_router(service_name: &str, region: &str) {
  let router = String::from(service_name) + "-router";
  let network = String::from("--network=") + service_name;
  let region_str = String::from("--region=") + region;
  let output = Command::new("gcloud")
    .args(&[
      "compute",
      "routers",
      "create",
      &router,
      &network,
      &region_str,
    ])
    .output()
    .await;
  println!("output = {:?}", output);
}

pub async fn process_create_external_ip(service_name: &str, region: &str) {
  let external_ip = String::from(service_name) + "-ip";
  let region_str = String::from("--region=") + region;
  let output = Command::new("gcloud")
    .args(&["compute", "addresses", "create", &external_ip, &region_str])
    .output()
    .await;
  println!("output = {:?}", output);
}

pub async fn process_create_nat(service_name: &str, region: &str) {
  let nat = String::from(service_name) + "-nat";
  let router = String::from("--router=") + service_name + "-router";
  let region_str = String::from("--region=") + region;
  let nat_custom_subnet_ip_ranges = String::from("--nat-custom-subnet-ip-ranges=") + service_name + "-subnet";
  let nat_external_ip_pool = String::from("--nat-external-ip-pool=") + service_name + "-ip";
  let output = Command::new("gcloud")
    .args(&[
      "compute",
      "routers",
      "nats",
      "create",
      &nat,
      &router,
      &region_str,
      &nat_custom_subnet_ip_ranges,
      &nat_external_ip_pool,
    ])
    .output()
    .await;
  println!("output = {:?}", output);
}
