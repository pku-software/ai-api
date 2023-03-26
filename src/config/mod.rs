use std::{fs::File, io::Read};

use serde::Deserialize;
use toml;

#[derive(Default, Debug, Clone, Deserialize)]
pub(crate) struct Translate {
    pub(crate) appid: String,
    pub(crate) secret: String,
    pub(crate) source: String,
    pub(crate) target: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub(crate) struct Chat {
    pub(crate) token: String,
    pub(crate) max_tokens: i32,
    pub(crate) temperature: f32,
    pub(crate) top_p: i32,
    pub(crate) frequency_penalty: f32,
    pub(crate) presence_penalty: f32,
    pub(crate) stop: Vec<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub(crate) struct Draw {
    pub(crate) token: String,
    pub(crate) number: i32,
    pub(crate) width: i32,
    pub(crate) height: i32,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub(crate) struct Math {
    pub(crate) token: String,
    pub(crate) width: i32,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub(crate) struct Config {
    pub(crate) mongo_uri: String,
    pub(crate) user_csv: String,
    pub(crate) translate: Translate,
    pub(crate) chat: Chat,
    pub(crate) draw: Draw,
    pub(crate) math: Math,
}

impl Config {
    pub fn new() -> Config {
        Config {
            mongo_uri: String::from("mongodb://localhost:27017"),
            user_csv: String::from("users.csv"),
            translate: Translate {
                appid: String::from(""),
                secret: String::from(""),
                source: String::from("en"),
                target: String::from("zh"),
            },
            chat: Chat {
                token: String::from(""),
                max_tokens: 100,
                temperature: 0.9,
                top_p: 1,
                frequency_penalty: 0.0,
                presence_penalty: 0.0,
                stop: vec![
                    "\n".to_owned(),
                    " Human:".to_owned().to_owned(),
                    " AI:".to_owned(),
                ],
            },
            draw: Draw {
                token: String::from(""),
                number: 1,
                width: 512,
                height: 512,
            },
            math: Math {
                token: String::from(""),
                width: 512,
            },
        }
    }

    pub fn from_toml(file: &str) -> Config {
        let mut f = File::open(file).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        match toml::from_str(&s) {
            Ok(config) => config,
            Err(e) => {
                println!("Error: {}", e);
                Config::new()
            }
        }
    }
}
