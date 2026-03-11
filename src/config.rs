use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub gemini_api_key: String,
    pub sample: String,
    pub default_connect_amount: i8,
    #[serde(default, alias = "default_interact_amount")]
    pub default_comment_amount: i8,
    pub rating_threshold: i32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            gemini_api_key: String::new(),
            sample: String::new(),
            default_connect_amount: 10,
            default_comment_amount: 5,
            rating_threshold: 7,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        match fs::read_to_string("config.json") {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
            Err(_) => {
                let config = Config::default();
                config.save();
                config
            }
        }
    }

    pub fn save(&self) {
        let contents = serde_json::to_string_pretty(self).unwrap();
        fs::write("config.json", contents).unwrap();
    }
}
