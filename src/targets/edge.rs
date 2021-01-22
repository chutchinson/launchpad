
use crate::config::{Config, ProfileConfig, BrowserConfig, WindowConfig};
use crate::launch::context::{LaunchContext, LaunchHandle, LaunchTarget};

use std::process::{Child, Command, ExitStatus};
use std::path::Path;

pub struct Edge;

lazy_static! {
    static ref WELL_KNOWN_LOCATIONS: Vec<&'static str> = vec![
        "C:/Program Files/Microsoft/Edge/Application/msedge.exe",
        "C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe",
    ];
}

impl Edge {
    pub fn create(_config: &Config) -> Self {
        Edge {}
    }
}

impl LaunchTarget for Edge {
    fn find(&self, context: &mut LaunchContext) -> Option<String> {
        context.find_file(&WELL_KNOWN_LOCATIONS)
    }
    fn launch(&self, context: &mut LaunchContext, profile: &ProfileConfig, path: &Path) -> Option<LaunchHandle> {
        let chrome = super::chrome::Chrome::create(&context.config);
        return chrome.launch(context, profile, path);
    }
}
