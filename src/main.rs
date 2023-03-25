use clap_v3::{App, Arg};
use config::Config;
use env_logger;

#[macro_use]
extern crate log;

pub(crate) mod config;
pub(crate) mod db;
pub(crate) mod handler;

fn main() {
    let matches = App::new("AI api server")
        .version("0.1")
        .author("Yuanhang Sun")
        .about("AI api server")
        .arg(
            Arg::with_name("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("bind")
                .short('b')
                .long("bind")
                .value_name("IP:PORT")
                .help("Sets a custom bind address")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("debug")
                .short('d')
                .long("debug")
                .help("Sets debug mode for log")
                .takes_value(false),
        )
        .get_matches();
    let config = matches.value_of("config").unwrap_or("config.toml");
    let bind = matches.value_of("bind").unwrap_or("0.0.0.0:4399");
    let debug = matches.is_present("debug");

    if debug {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    info!("Starting AI api server...");

    info!("Config file: {}", config);
    info!("Bind to: {}", bind);
    info!("Debug mode: {}", debug);

    let config = Config::from_toml(config);

    let db = db::SQLiteDatebase::new(&config).unwrap();
    db.init(&config).unwrap();
}
