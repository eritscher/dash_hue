use pnet::util::MacAddr;

#[allow(unused_variables)]
pub trait Events {
  fn on_arp(&self, address: MacAddr) {}
  fn on_ipv4(&self, address: MacAddr) {}
  fn on_ipv6(&self, address: MacAddr) {}
}
