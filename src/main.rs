#[macro_use]
extern crate log;

use env_logger::Env;
use structopt::StructOpt;

/// Display list of pods in Kubernetes namespace
#[derive(StructOpt)]
struct Cli {
    /// The Kubernetes namespace
    namespace: String,
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    let args = Cli::from_args();
    info!("Getting pods for namespace {}", &args.namespace);
}
