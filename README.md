# Launchpad

A utility that can launch web applications, software, and other executables with a user-friendly configuration. 

## Features

- Launch websites with a specific browser (if installed)
    - Google Chrome
    - Microsoft Edge
- Configurable
    - Kiosk mode (attempts to configure application to appear in kiosk mode)
    - Display / monitor selection (moves application window to specific monitor)
    - Vendor-agnostic configuration properties

# Goals

- Easy installation (single executable)
- Easier configuration (avoid JSON, XML, etc)

## Future Goals

- Cross-platform support 
- Application version management
    - Automatically search for new application versions (and download them)
    - Automatic installation for certain installation types
- Deployment pipelines
    - Extract-Transform-Load (ETL) style pipelines for launching apps
- Docker / Kubernetes support
- Launch templates
- Embedded Scripting

## Supported Targets

| Name                                | Identifier |
| ----------------------------------- | ---------- |
| Google Chrome                       | chrome     |
| Microsoft Edge                      | edge       |

## Configuration

Configuration is done using the TOML language: 
https://github.com/toml-lang/toml

```toml
# Launches 2 Chrome windows in kiosk and incognito mode on separate monitors

[[profile]]
target = 'chrome'
url = 'http://www.google.com'
display = 0
kiosk = true
browser.incognito = true

[[profile]]
target = 'chrome'
url = 'http://www.microsoft.com'
display = 1
kiosk = true
browser.incognito = true
```