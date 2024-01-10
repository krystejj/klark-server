use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, write};
use std::path::Path;

const CONFIG_FILE_PATH: &str = "config.toml";

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
  pub port: u16,
}

impl Default for Config {
  fn default() -> Self {
    Config { port: 4040 }
  }
}

pub fn init_config() -> Config {
  // Create config file if it doesn't exist and return default
  if !Path::new(CONFIG_FILE_PATH).exists() {
    let default_conf = Config::default();
    write(
      CONFIG_FILE_PATH,
      toml::to_string(&default_conf).expect("couldn't parse default config struct"),
    )
    .expect("couldn't write default config file");
    return default_conf;
  }

  // Read config from a file and return it
  let config_text = read_to_string(CONFIG_FILE_PATH).expect("couldn't read config file");
  toml::from_str(&config_text).expect("couldn't parse config file")
}
