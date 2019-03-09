use pnet::util::MacAddr;

pub trait Events {
  fn on_arp(&self, source_address: MacAddr) {}
  fn on_ipv4(&self, source_address: MacAddr) {}
  fn on_ipv6(&self, source_address: MacAddr) {}
}
