#[macro_use]
extern crate objc;

// Use better Apple logging support? https://lib.rs/crates/oslog
mod audio;
mod config;
mod event_loop;
mod popup;
mod popup_content;
mod shortcuts;
mod tray;
mod ui;
mod utils;

use audio::AudioController;
use env_logger::{Builder, Env};
use event_loop::start;
use log::{info, trace};
use ui::UI;
use utils::arc_lock;

fn main() {
    Builder::from_env(Env::default().default_filter_or("trace")).init();
    info!("Starting app");
    let controller = arc_lock(AudioController::new().unwrap());
    trace!("Controller initialized {:?}", controller);
    let controller_main = controller.clone();
    let controller_loop = controller.clone();
    let (ui, event_loop, event_ids) = UI::new(controller_main).unwrap();
    trace!("UI initialized");
    let ui = arc_lock(ui);
    start(event_loop, event_ids, ui, controller_loop);
}
