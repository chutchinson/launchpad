use std::path::{Path, PathBuf};
use std::process::{Child};

use crate::config::{Config, ProfileConfig};
use crate::targets::chrome::Chrome;

use winit::window::{Window, WindowBuilder};
use winit::event_loop::EventLoop;
use context::LaunchContext;

pub mod context;
pub use context::*;

#[derive(Debug)]
pub struct Monitor {
    pub pos: (i32, i32),
    pub size: (u32, u32)
}

pub enum LaunchError {
    Unknown,
    NotFound
}

pub fn launch(config: Config) -> Result<(), LaunchError> {

    let mut context = LaunchContext::new(config);

    let profiles: Vec<_> = context.config.profile.iter()
        .map(|p| p.clone())
        .collect();

    for profile in profiles.iter() {
        if let Some(target) = context.get_target(profile) {
            let path = target.find(&mut context);

            if path.is_none() {
                return Err(LaunchError::NotFound);
            }

            let path = PathBuf::from(path.unwrap());
            let profile = profile.clone();

            if let Some(child) = target.launch(&mut context, &profile, &path) {
                context.handles.push(child);
            }
        }
        else {
            return Err(LaunchError::NotFound);
        }
    }

    // wait for children

    for child in context.handles.iter_mut() {
        child.wait();
        child.kill();
    }

    return Err(LaunchError::Unknown);
}
