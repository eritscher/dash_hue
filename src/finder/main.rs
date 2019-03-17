use pnet::datalink::{self, Channel, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};

fn main() {
  let mut seen_macs = Vec::new();

  let interface = datalink::interfaces()
    .into_iter()
    .filter(|iface: &NetworkInterface| iface.name == "en0")
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
        match packet.get_ethertype() {
          EtherTypes::Arp => {
            if !seen_macs.contains(&packet.get_source()) {
              println!("ARP PACKET FOUND: \n{}", packet.get_source());
              seen_macs.push(packet.get_source());
            };
          }
          EtherTypes::Ipv4 => {
            if !seen_macs.contains(&packet.get_source()) {
              println!("IPv4 PACKET FOUND: \n{}", packet.get_source());
              seen_macs.push(packet.get_source());
            };
          }
          _ => {}
        }
      }
      Err(e) => panic!("An error occured recieving from the datalink, {:?}", e),
    }
  }
}
