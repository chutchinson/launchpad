#[macro_use] extern crate lazy_static;

mod launch;
mod config;
mod platform;
mod targets;

use config::Config;
use launch::launch;

fn main() {
    let mut config_path = std::path::PathBuf::from("launch");
    let mut config = Config::default();

    let mut args = std::env::args();
    let arg = args.skip(1).next();

    if arg.is_some() {
        config_path.clear();
        config_path.push(arg.unwrap());
    }

    if config_path.exists() {
        config = Config::load(config_path).unwrap();
    }

    launch::launch(config);
}
