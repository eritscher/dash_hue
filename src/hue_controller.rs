use crate::events::Events;
use pnet::util::MacAddr;

pub struct HueController;

impl Events for HueController {
  fn on_arp(&self, source_mac_address: MacAddr) {
    println!("Hi there, received a ARP frame");
  }
  fn on_ipv4(&self, source_mac_address: MacAddr) {
    println!("Hi there, received a IP4 frame");
  }
  fn on_ipv6(&self, source_mac_address: MacAddr) {
    println!("Hi there, received a IP6 frame");
  }
}
