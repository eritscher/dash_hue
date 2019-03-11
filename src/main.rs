mod button;
mod config;
mod events;
mod hue_controller;
mod listener;

use config::Config;
use hue_controller::HueController;
use listener::PacketListener;

fn main() {
    let tracked_buttons = &Config.buttons;
    let mut listener = PacketListener::new(&tracked_buttons);
    let hue_controller = HueController::new(&tracked_buttons);
    listener.add_events_hook(hue_controller);
    listener.start();
}
