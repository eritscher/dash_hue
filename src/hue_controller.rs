use crate::button::Button;
use crate::events::Events;
use pnet::util::MacAddr;

pub struct HueController {
  buttons: &'static Vec<Button>,
}
impl HueController {
  pub fn new(buttons: &'static Vec<Button>) -> HueController {
    HueController { buttons: &buttons }
  }
}

impl Events for HueController {
  fn on_arp(&self, address: MacAddr) {
    println!("Hi there, received a ARP frame");
  }
  fn on_ipv4(&self, address: MacAddr) {
    println!("Hi there, received a IP4 frame from {}", address);
  }
  fn on_ipv6(&self, address: MacAddr) {
    println!("Hi there, received a IP6 frame");
  }
}
