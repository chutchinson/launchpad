use std::path::{Path, PathBuf};
use std::process::{Child};

use crate::config::{Config, ProfileConfig};
use crate::targets::chrome::Chrome;

use winit::window::{Window, WindowBuilder};
use winit::event_loop::EventLoop;
use crate::launch::{Monitor};

pub trait LaunchTarget {
    fn find(&self, context: &mut LaunchContext) -> Option<String>;
    fn launch(&self, context: &mut LaunchContext, profile: &ProfileConfig, path: &Path) -> Option<LaunchHandle>;
}

pub struct LaunchContext {
    pub config: Config,
    pub handles: Vec<LaunchHandle>,
    events: EventLoop<()>,
    window: Window
}

pub struct LaunchHandle {
    child: Option<Child>,
    temp_path: Option<String>
}

impl LaunchHandle {
    pub fn new(child: Option<Child>, path: Option<String>) -> Self {
        LaunchHandle {
            child,
            temp_path: path
        }
    }
    pub fn wait(&mut self) {
        if let Some(child) = &mut self.child {
            child.wait();
        }
    }
    pub fn kill(&mut self) {
        if let Some(child) = &mut self.child {
            child.kill();
        }
        // delete temporary files
        if let Some(path) = self.temp_path.as_ref() {
            std::fs::remove_dir_all(path);
        }
    }
}

impl LaunchContext {
    pub fn new(config: Config) -> Self {
        let events = EventLoop::new();
        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&events)
            .unwrap();
        let handles = Vec::new();
        LaunchContext {
            config,
            handles,
            events,
            window
        }
    }
    pub fn get_monitor(&self, index: usize) -> Option<Monitor> {
        let monitors: Vec<_> = self.window.available_monitors().collect();
        monitors.get(index).map(|x| {
            let pos = x.position();
            let size = x.size();
            Monitor {
                pos: (pos.x, pos.y),
                size: (size.width, size.height)
            }
        })
    }
    pub fn get_target(&self, profile: &ProfileConfig) -> Option<Box<dyn LaunchTarget>> {
        use crate::targets::chrome::Chrome;
        use crate::targets::edge::Edge;

        if let Some(name) = profile.target.as_ref() {
            return match name.as_str() {
                "chrome" => Some(Box::new(Chrome::create(&self.config))),
                "edge" => Some(Box::new(Edge::create(&self.config))),
                _ => None
            };
        }
        None
    }
    pub fn find_file(&self, paths: &Vec<&str>) -> Option<String> {
        for path in paths {
            let path = PathBuf::from(path);
            if path.exists() {
                return path.as_path().to_str().map(|v| v.to_owned())
            }
        }
        return None
    }

    pub fn move_window(&self, pid: u32, index: usize) {
        use crate::platform::win32::*;
        if let Some(monitor) =  self.get_monitor(index) {
            let (x, y) = monitor.pos;
            let (width, height) = monitor.size;
            if let Some(hwnd) = find_main_window(pid) {
                move_window(hwnd, x, y, width, height);
            }
        }
    }
}
