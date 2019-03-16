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

fn get_command_line_args() -> clap::ArgMatches<'static> {
  return App::new("dash_hue")
    .arg(
      Arg::with_name("config-file")
        .short("c")
        .long("config-file")
        .help("Location of file containing button list json.")
        .required(true)
        .takes_value(true),
    )
    .get_matches();
}

pub struct ConfigManager {
  pub buttons: Vec<Button>,
  config: Value,
  pub host: String,
  pub api_key: String,
}

impl ConfigManager {
  pub fn new() -> ConfigManager {
    let mut tracked_buttons: Vec<Button> = Vec::new();
    let matches = get_command_line_args();
    let path = Path::new(matches.value_of("config-file").unwrap());
    let config: Value = serde_json::from_str(fs::read_to_string(path).unwrap().as_str()).unwrap();

    if let Some(button_list) = config.get("buttons") {
      match button_list {
        Value::Array(btns) => {
          for btn in btns {
            let address = MacAddr::from_str(btn.get("address").unwrap().as_str().unwrap()).unwrap();
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
    let host = config.get("host").unwrap().as_str().unwrap().to_owned();
    let api_key = config.get("APIKey").unwrap().as_str().unwrap().to_owned();

    return ConfigManager {
      buttons: tracked_buttons,
      host: host,
      api_key: api_key,
      config: config,
    };
  }
}

lazy_static! {
  pub static ref CONFIG: ConfigManager = { ConfigManager::new() };
  pub static ref PUSH_TIMES: Mutex<HashMap<MacAddr, Instant>> = { Mutex::new(HashMap::new()) };
}
