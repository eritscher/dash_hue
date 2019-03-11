use crate::button::Button;
use crate::events::Events;
use pnet::util::MacAddr;
use std::iter::FromIterator;
pub struct HueController {
  buttons: &'static Vec<Button>,
}
impl HueController {
  pub fn new(buttons: &'static Vec<Button>) -> HueController {
    HueController { buttons: &buttons }
  }
  fn get_target(&self, address: MacAddr) -> &Button {
    let target: &Button = self
      .buttons
      .iter()
      .filter(|&btn| btn.address == address)
      .collect::<Vec<&Button>>()
      .first()
      .unwrap();
    return target;
  }
}

impl Events for HueController {
  fn on_arp(&self, address: MacAddr) {
    println!("Hi there, received a ARP frame");
  }
  fn on_ipv4(&self, address: MacAddr) {
    println!("Hi there, received a IP4 frame from {}", address);
    let target = self.get_target(address);
  }
  fn on_ipv6(&self, address: MacAddr) {
    println!("Hi there, received a IP6 frame");
  }
}
