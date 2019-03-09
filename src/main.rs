extern crate pnet;
use pnet::util::MacAddr;
use std::str::FromStr;
mod events;
mod hue_controller;
mod listener;
use hue_controller::HueController;
use listener::PacketListener;

fn main() {
    let mut vec = Vec::new();
    let address: MacAddr = MacAddr::from_str("00:00:00:00:00:00").unwrap();
    vec.push(address);
    let mut listener = PacketListener::new(vec);
    listener.add_events_hook(HueController);
    listener.start();
}
