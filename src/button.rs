use pnet::util::MacAddr;

pub struct Button {
  pub address: MacAddr,
  pub name: String,
  pub desc: String,
}
