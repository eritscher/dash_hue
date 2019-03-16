use crate::button::Button;
use crate::config::CONFIG;
use crate::events::Events;
use crate::room::Room;
use pnet::util::MacAddr;
use reqwest::Client;
use std::collections::HashMap;

pub struct HueController {
  http_client: Client,
  tracked_buttons: &'static Vec<Button>,
}
impl HueController {
  pub fn new(tracked_buttons: &'static Vec<Button>) -> HueController {
    HueController {
      http_client: Client::new(),
      tracked_buttons: &tracked_buttons,
    }
  }

  fn get_pressed_button(&self, address: MacAddr) -> &Button {
    self
      .tracked_buttons
      .iter()
      .filter(|&btn| btn.address == address)
      .collect::<Vec<&Button>>()
      .first()
      .unwrap()
  }

  fn toggle_room_state(&self, group_id: &str) {
    let get_room_url = format!(
      "http://{}/api/{}/groups/{}",
      &CONFIG.host.as_str(),
      &CONFIG.api_key.as_str(),
      group_id
    );

    let current_room_state: Room = self
      .http_client
      .get(get_room_url.as_str())
      .send()
      .unwrap()
      .json::<Room>()
      .unwrap();

    let mut payload: HashMap<String, bool> = HashMap::new();
    payload.insert("on".to_owned(), true);

    if current_room_state.state.any_on {
      *payload.get_mut("on").unwrap() = false;
    };
    let put_room_url = format!(
      "http://{}/api/{}/groups/{}/action",
      &CONFIG.host.as_str(),
      &CONFIG.api_key.as_str(),
      group_id
    );

    let body = serde_json::to_string(&payload).unwrap();
    let update_room_state = self
      .http_client
      .put(put_room_url.as_str())
      .body(body)
      .send();
  }
}

impl Events for HueController {
  fn on_arp(&self, address: MacAddr) {
    println!("Hi there, received a ARP frame");
  }
  fn on_ipv4(&self, address: MacAddr) {
    let pressed_button = self.get_pressed_button(address);
    self.toggle_room_state(&pressed_button.room);
  }
  fn on_ipv6(&self, address: MacAddr) {
    println!("Hi there, received a IP6 frame");
  }
}
