use crate::button::Button;
use clap::{App, Arg};
use lazy_static::lazy_static;
use pnet::util::MacAddr;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::sync::Mutex;
use std::time::Instant;

lazy_static! {
  pub static ref CONFIG: ConfigManager = { ConfigManager::new() };
  pub static ref PUSH_TIMES: Mutex<HashMap<MacAddr, Instant>> = { Mutex::new(HashMap::new()) };
}

pub struct ConfigManager {
  pub buttons: Vec<Button>,
  pub host: String,
  pub api_key: String,
  pub interface: String,
}

impl ConfigManager {
  pub fn new() -> ConfigManager {
    let mut tracked_buttons: Vec<Button> = Vec::new();
    let matches = get_command_line_args();
    let path = Path::new(matches.value_of("button-config").unwrap());
    let button_config: Value =
      serde_json::from_str(fs::read_to_string(path).unwrap().as_str()).unwrap();

    if let Some(button_list) = button_config.get("buttons") {
      match button_list {
        Value::Array(btns) => {
          for btn in btns {
            let addres_str = btn
              .get("address")
              .expect("A button did not include a mac address.")
              .as_str()
              .unwrap();
            let address = MacAddr::from_str(addres_str).unwrap();
            let name = btn
              .get("name")
              .expect("A button was not given a canonical name.")
              .as_str()
              .unwrap()
              .to_owned();
            let desc: Option<String> = match btn.get("desc") {
              Some(desc) => Some(desc.as_str().unwrap().to_owned()),
              None => None,
            };
            let room = btn
              .get("room")
              .expect("A room id was not associated with a button.")
              .as_str()
              .unwrap()
              .to_owned();

            let button = Button {
              address,
              name,
              desc,
              room,
            };
            tracked_buttons.push(button);
          }
        }
        _ => panic!("The value of \"buttons\" in your config file is not an array"),
      }
    }

    let host = matches.value_of("host").unwrap();
    let api_key = matches.value_of("api-key").unwrap();
    let interface = matches.value_of("interfaces").unwrap();

    return ConfigManager {
      buttons: tracked_buttons,
      host: host.to_string(),
      api_key: api_key.to_string(),
      interface: interface.to_string(),
    };
  }
}

fn get_command_line_args() -> clap::ArgMatches<'static> {
  return App::new("dash_hue")
    .arg(
      Arg::with_name("button-config")
        .short("b")
        .long("button-config")
        .help("Location of file containing button list json.")
        .required(true)
        .takes_value(true),
    )
    .arg(
      Arg::with_name("host")
        .short("h")
        .long("host")
        .help("Hue Bridge IP")
        .required(true)
        .takes_value(true),
    )
    .arg(
      Arg::with_name("api-key")
        .short("a")
        .long("api-key")
        .help("Hue Bridge API key.")
        .required(true)
        .takes_value(true),
    )
    .arg(
      Arg::with_name("interfaces")
        .short("i")
        .long("interfaces")
        .help("Space separated list of network interfaces on which to listen.")
        .takes_value(true)
        .default_value("en0"),
    )
    .get_matches();
}
