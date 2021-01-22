

use crate::config::{Config, ProfileConfig};
use crate::launch::context::{LaunchContext, LaunchHandle, LaunchTarget};

use std::process::{Command};
use std::path::Path;

pub struct Chrome;

lazy_static! {
    static ref WELL_KNOWN_LOCATIONS: Vec<&'static str> = vec![
        "C:/Program Files/Google/Chrome/Application/chrome.exe",
        "C:/Program Files (x86)/Google/Chrome/Application/chrome.exe"
    ];
}

impl Chrome {
    pub fn create(_config: &Config) -> Self {
        Chrome {}
    }
}

impl LaunchTarget for Chrome {
    fn find(&self, context: &mut LaunchContext) -> Option<String> {
        context.find_file(WELL_KNOWN_LOCATIONS.as_ref())
    }

    fn launch(&self, context: &mut LaunchContext, profile: &ProfileConfig, path: &Path) -> Option<LaunchHandle> {
        let mut command = Command::new(path);

        let config = profile;
        let url = config.url.as_ref();
        let browser = config.browser.as_ref();
        let window = config.window.as_ref();
        let mut is_distinct = true;

        if let Some(monitor_index) = config.display {
            if let Some(monitor) = context.get_monitor(monitor_index) {
                let (x, y) = monitor.pos;
                let (width, height) = monitor.size;
                command.arg(format!("--window-position={},{}", x, y));
                command.arg(format!("--window-size={},{}", width, height));
            }
        }

        if config.kiosk.unwrap_or(false) {
            command.arg("--kiosk");
        }

        if let Some(window) = window {
            if window.x.is_some() && window.y.is_some() {
                command.arg(format!("--window-position={},{}", window.x.unwrap(), window.y.unwrap()));
            }
            if window.width.is_some() && window.height.is_some() {
                command.arg(format!("--window-size={},{}", window.width.unwrap(), window.height.unwrap()));
            }
        }

        if let Some(browser) = browser {
            if browser.incognito.unwrap_or(false) {
                command.arg("--incognito");
            }
            if !browser.distinct.unwrap_or(is_distinct) {
                is_distinct = false;
            }
        }

        if is_distinct {
            command.arg("--new-window");
        }

        if let Some(url) = url {
            command.arg(url);
        }

        // generate random temporary directory for browser data

        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        let random_id = rng.gen::<u32>();

        let mut user_dir = std::path::PathBuf::new();
        user_dir.push(std::env::temp_dir());
        user_dir.push(format!("{}", random_id));

        let path = user_dir.to_str().unwrap();
        command.arg(format!("--user-data-dir={}", path));

        // spawn browser process

        let child = command.spawn();

        if child.is_ok() {
            let path = user_dir.to_str().map(|x| x.to_string());
            let handle = LaunchHandle::new(child.ok(), path);
            return Some(handle);
        }

        None
    }
}