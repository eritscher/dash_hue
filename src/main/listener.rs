use crate::button::Button;
use crate::config::{CONFIG, PUSH_TIMES};
use crate::events::Events;
use pnet::datalink::{self, Channel, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use std::time::{Duration, Instant};

pub struct PacketListener {
  tracked_buttons: &'static Vec<Button>,
  hooks: Vec<Box<dyn Events>>,
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
      .filter(|iface: &NetworkInterface| iface.name == CONFIG.interface)
      .next()
      .expect("Unable to detect selected network interface.");

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
        Err(e) => panic!("An error occured recieving from the datalink, {:?}", e),
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
      match PUSH_TIMES.lock() {
        Ok(mut push_times) => {
          let last_push = push_times.get(&packet_mac);
          if last_push.is_none()
            || (last_push.is_some() && last_push.unwrap().elapsed() > Duration::from_secs(5))
          {
            push_times.insert(packet_mac, Instant::now());
          } else {
            return;
          }
        }
        Err(e) => println!("error {}", e),
      }
      #[allow(unused_doc_comments)]
      /**
       * For some reason the raspberry pi is not able to read the packet's ethertype. It always comes in as unknown.
       * For now, I'm going to just remove this and always respond with the ipv4 hook on a matched packet.
       */
      for hook in &self.hooks {
        hook.on_ipv4(packet_mac);
      }

      // match packet.get_ethertype() {
      //   EtherTypes::Arp => {
      //     for hook in &self.hooks {
      //       hook.on_arp(packet_mac);
      //     }
      //   }
      //   EtherTypes::Ipv4 => {
      //     for hook in &self.hooks {
      //       hook.on_ipv4(packet_mac);
      //     }
      //   }
      //   EtherTypes::Ipv6 => {
      //     for hook in &self.hooks {
      //       hook.on_ipv6(packet_mac);
      //     }
      //   }
      //   _ => {}
      // }
    }
  }
}
