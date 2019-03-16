use crate::button::Button;
use crate::config::PUSH_TIMES;
use crate::events::Events;
use pnet::datalink::{self, Channel, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use std::time::{Duration, Instant};

pub struct PacketListener {
  tracked_buttons: &'static Vec<Button>,
  hooks: Vec<Box<Events>>,
}

impl PacketListener {
  pub fn new(tracked_buttons: &'static Vec<Button>) -> PacketListener {
    PacketListener {
      tracked_buttons: &tracked_buttons,
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
    let mut is_tracked_button = false;
    for button in self.tracked_buttons {
      if button.address == packet_mac {
        is_tracked_button = true;
        break;
      }
    }
    if is_tracked_button {
      match packet.get_ethertype() {
        EtherTypes::Arp => {
          for hook in &self.hooks {
            hook.on_arp(packet_mac);
          }
        }
        EtherTypes::Ipv4 => {
          let lock = PUSH_TIMES.lock();
          match lock {
            Ok(mut push_times) => {
              if let Some(last_push) = push_times.get(&packet_mac) {
                if last_push.elapsed() > Duration::from_secs(5) {
                  for hook in &self.hooks {
                    hook.on_ipv4(packet_mac);
                  }
                  push_times.insert(packet_mac, Instant::now());
                } else {
                  println!("Received packet, debouncing.");
                }
              } else {
                push_times.insert(packet_mac, Instant::now());
                for hook in &self.hooks {
                  hook.on_ipv4(packet_mac);
                }
              }
            }
            Err(e) => println!("error {}", e),
          }
        }
        EtherTypes::Ipv6 => {
          for hook in &self.hooks {
            hook.on_ipv6(packet_mac);
          }
        }
        _ => {}
      }
    }
  }
}
