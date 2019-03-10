#[macro_use]
extern crate lazy_static;
extern crate pnet;
use pnet::util::MacAddr;
use std::str::FromStr;
mod button;
mod events;
mod hue_controller;
mod listener;
use button::Button;
use hue_controller::HueController;
use listener::PacketListener;

lazy_static! {
    pub static ref BUTTONS: Vec<Button> = {
        let mut button_vec = Vec::new();
        let button1 = Button {
            address: MacAddr::from_str("00:00:00:00:00:00").unwrap(),
            name: "hi".to_owned(),
            desc: "hi".to_owned(),
        };
        button_vec.push(button1);
        button_vec
    };
}
fn main() {
    let mut listener = PacketListener::new(&BUTTONS);
    let hue_controller = HueController::new(&BUTTONS);
    listener.add_events_hook(hue_controller);
    listener.start();
}
