use std::{fs::File, io::Read};

use serde::Deserialize;
use toml;

#[derive(Default, Debug, Deserialize)]
pub(crate) struct Translate {
    pub(crate) source: String,
    pub(crate) target: String,
}

#[derive(Default, Debug, Deserialize)]
pub(crate) struct Chat {
    pub(crate) max_tokens: i32,
    pub(crate) temperature: f32,
    pub(crate) top_p: i32,
    pub(crate) frequency_penalty: f32,
    pub(crate) presence_penalty: f32,
    pub(crate) stop: Vec<String>,
}

#[derive(Default, Debug, Deserialize)]
pub(crate) struct Draw {
    pub(crate) number: i32,
    pub(crate) width: i32,
    pub(crate) height: i32,
}

#[derive(Default, Debug, Deserialize)]
pub(crate) struct Math {
    pub(crate) width: i32,
}

#[derive(Default, Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) db_file: String,
    pub(crate) user_csv: String,
    pub(crate) openai_token: String,
    pub(crate) wolfram_token: String,
    pub(crate) max_request: i32,
    pub(crate) max_request_per_user: i32,
    pub(crate) translate: Translate,
    pub(crate) chat: Chat,
    pub(crate) draw: Draw,
    pub(crate) math: Math,
}

impl Config {
    pub fn new() -> Config {
        Config {
            db_file: String::from("db.sqlite3"),
            user_csv: String::from("users.csv"),
            openai_token: String::from(""),
            wolfram_token: String::from(""),
            max_request: 100,
            max_request_per_user: 10,
            translate: Translate {
                source: String::from("en"),
                target: String::from("zh"),
            },
            chat: Chat {
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
                number: 1,
                width: 512,
                height: 512,
            },
            math: Math { width: 512 },
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
