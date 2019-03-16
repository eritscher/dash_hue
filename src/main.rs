mod button;
mod config;
mod events;
mod hue_controller;
mod listener;
mod room;

use config::CONFIG;
use hue_controller::HueController;
use listener::PacketListener;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

fn main() {
    let tracked_buttons = &CONFIG.buttons;
    let mut listener = PacketListener::new(&tracked_buttons);
    let hue_controller = HueController::new(&tracked_buttons);
    listener.add_events_hook(hue_controller);
    listener.start();
}
