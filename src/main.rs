#[macro_use] extern crate lazy_static;

mod launch;
mod config;
mod targets;

use config::Config;
use crate::launch::LaunchError;

fn run() -> Result<(), LaunchError> {
    let mut config_path = std::path::PathBuf::from("launch");
    let mut config = Config::default();

    let args = std::env::args();
    let arg = args.skip(1).next();

    if arg.is_some() {
        config_path.clear();
        config_path.push(arg.unwrap());
    }

    if config_path.exists() {
        config = Config::load(config_path).expect("failed to load configuration");
    }

    launch::launch(config)
}

fn main() {
    // TODO: error codes
    let exit_code = match run() {
        Ok(_) => 0,
        Err(_) => -1
    };

    std::process::exit(exit_code);
}
