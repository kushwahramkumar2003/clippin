//! ClipPin — macOS menu-bar clipboard history manager.
//!
//! Entry point: initializes logging and starts the AppKit application.

mod accessibility;
mod app;
mod autopaste;
mod clipboard;
mod hotkey;
mod launch;
mod privacy;
mod settings;
mod status_item;
mod storage;

use log::info;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("ClipPin starting");
    app::run();
}
