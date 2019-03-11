use crate::button::Button;
use clap::{App, Arg};
use lazy_static::lazy_static;
use pnet::util::MacAddr;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::str::FromStr;

fn make_command_line_parser() -> clap::ArgMatches<'static> {
  return App::new("dash_hue")
    .arg(
      Arg::with_name("json-file")
        .short("j")
        .long("json-file")
        .help("Location of file containing button list json.")
        .required(true)
        .takes_value(true),
    )
    .get_matches();
}

pub struct ConfigManager {
  pub buttons: Vec<Button>,
}

impl ConfigManager {
  pub fn new() -> ConfigManager {
    let mut tracked_buttons: Vec<Button> = Vec::new();
    let matches = make_command_line_parser();
    let file = matches.value_of("json-file").unwrap();
    let path = Path::new(file);
    let json: Value = serde_json::from_str(fs::read_to_string(path).unwrap().as_str()).unwrap();

    if let Some(btn) = json.get("buttons") {
      match btn {
        Value::Array(btn) => {
          for b in btn {
            let address = MacAddr::from_str(b.get("address").unwrap().as_str().unwrap()).unwrap();
            let button = Button {
              address,
              name: b.get("name").unwrap().as_str().unwrap().to_owned(),
              desc: b.get("desc").unwrap().as_str().unwrap().to_owned(),
            };
            tracked_buttons.push(button);
          }
        }
        _ => {}
      }
    }
    return ConfigManager {
      buttons: tracked_buttons,
    };
  }
}

lazy_static! {
  pub static ref Config: ConfigManager = { ConfigManager::new() };
}
