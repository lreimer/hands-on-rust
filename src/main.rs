#[macro_use]
extern crate log;

use env_logger::Env;
use structopt::StructOpt;

use serde::{Deserialize};

/// Display list of pods in Kubernetes namespace
#[derive(StructOpt)]
struct Cli {
    /// The Kubernetes namespace
    namespace: String,
}

/// JSON definition of a PodList
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PodList {
    kind: String,
    api_version: String,
    items: Vec<Item>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Item {
    metadata: Metadata,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    let args = Cli::from_args();
    info!("Getting pods for namespace {}", &args.namespace);

    let request_url = format!("http://localhost:8001/api/v1/namespaces/{namespace}/pods",
                              namespace = &args.namespace);
    let response = reqwest::get(&request_url).await?;

    let pods: PodList = response.json().await?;
    println!("{:?}", pods);
    Ok(())
}
