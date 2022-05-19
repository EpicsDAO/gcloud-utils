<p align="center">
  <a href="https://twitter.com/intent/follow?screen_name=EpicsDAO">
    <img src="https://img.shields.io/twitter/follow/EpicsDAO.svg?label=Follow%20@EpicsDAO" alt="Follow @EpicsDAO" />
  </a>
  <br/>
  <a aria-label="Crate Version" href="https://crates.io/crates/gcloud-utils">
    <img alt="" src="https://badgen.net/crates/v/gcloud-utils">
  </a>
  <a aria-label="Crate Download" href="https://crates.io/crates/gcloud-utils">
    <img alt="" src="https://badgen.net/crates/d/gcloud-utils">
  </a>
  <a aria-label="License" href="https://github.com/EpicsDao/epics/blob/master/LICENSE.txt">
    <img alt="" src="https://badgen.net/badge/license/Apache/blue">
  </a>
    <a aria-label="Code of Conduct" href="https://github.com/EpicsDao/epics/blob/master/CODE_OF_CONDUCT.md">
    <img alt="" src="https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg">
  </a>
</p>


## What is gcloud-utils?
Google Cloud gcloud SDK Rust Wrapper.
Easy setup with Cloud Run for Web 2.5

## Installation
```bash
cargo install gcloud-utils
```

Create `gcp_config.json` config file at first.
```bash
gcu init config
```

`gcp_config.json`
```bash
{
  "project_id": "epic-app",
  "service_name": "epic-gcp",
  "region": "europe-west4"
}
```

## Usage

```bash
gcu --help
USAGE:
    gcu <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    compute    
    docker     
    gh         
    help       Print this message or the help of the given subcommand(s)
    iam        
    init       
    run   
```

## Google Cloud IAM Setup
Generate Service Account and Get Perimissions.

```bash
gcu iam setup
```

## Build Your Contaier
*make sure that you have a Dockerfile and it's successfully build.

### Using Docker
```bash
gcu docker build
gcu docker push
```

### Using gcloud build
```bash
gcu run build
```


## Google Cloud Run Deploy
*make sure that you have a Dockerfile and it's successfully build.

```bash
gcu run deploy
```


## Creating a VPC network

Make the necessary settings to scale the network, such as VPC network and firewall settings.

```bash
$ gcu compute create-nat
```

By this command

- Creating a VPC network
- Creating Firewall TCP rules
- Creating Firewall SSH rules
- Creating a network subnet
- Creating a VPC access connector
- Creating a Router
- Obtaining an External IP
- Creating Cloud NAT

Is done automatically.

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/EpicsDao/gcloud-utils. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [Contributor Covenant](http://contributor-covenant.org) code of conduct.

## License

The gem is available as open source under the terms of the [Apache-2.0 License](https://www.apache.org/licenses/LICENSE-2.0).

## Code of Conduct

Everyone interacting in the SOULs project’s codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](https://github.com/EpicsDao/gcloud-utils/blob/master/CODE_OF_CONDUCT.md).
