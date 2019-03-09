use crate::events::Events;
use pnet::datalink::{self, Channel, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::util::MacAddr;

pub struct PacketListener {
  mac_addresses: Vec<MacAddr>,
  hooks: Vec<Box<Events>>,
}

impl PacketListener {
  pub fn new(addresses: Vec<MacAddr>) -> Self {
    Self {
      mac_addresses: addresses,
      hooks: Vec::new(),
    }
  }
  pub fn add_events_hook<E: Events + 'static>(&mut self, hook: E) {
    self.hooks.push(Box::new(hook))
  }

  pub fn start(&self) {
    let interface = datalink::interfaces()
      .into_iter()
      .filter(|iface: &NetworkInterface| iface.name == "en0")
      .next()
      .unwrap();

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
      Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
      Ok(_) => panic!("An unknown channel"),
      Err(e) => panic!("error occured, {}", e),
    };

    loop {
      match rx.next() {
        Ok(raw_packet) => {
          let packet = EthernetPacket::new(raw_packet).unwrap();
          self.handle_ethernet_packet(&packet);
        }
        Err(e) => panic!("an error occured, {}", e),
      }
    }
  }

  fn handle_ethernet_packet(&self, packet: &EthernetPacket) {
    let packet_mac = packet.get_source();
    if self.mac_addresses.contains(&packet_mac) {
      match packet.get_ethertype() {
        EtherTypes::Arp => {
          for hook in &self.hooks {
            hook.on_arp(packet_mac);
          }
        }
        EtherTypes::Ipv4 => {
          for hook in &self.hooks {
            hook.on_ipv4(packet_mac);
          }
        }
        EtherTypes::Ipv6 => {
          for hook in &self.hooks {
            hook.on_ipv6(packet_mac);
          }
        }
        _ => {}
      }
    } else {
      println!("That's not a bing0");
    }
  }
}
