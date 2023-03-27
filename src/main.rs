use clap_v3::{App, Arg};
use config::Config;
use env_logger;
use once_cell::sync::Lazy;
use warp::Filter;
#[macro_use]
extern crate log;

pub(crate) mod config;
pub(crate) mod db;
pub(crate) mod handler;
pub(crate) mod openai;
pub(crate) mod translate;
pub(crate) mod wolfram;

pub(crate) static CONFIG: Lazy<Config> = Lazy::new(|| Config::from_toml("config.toml"));

#[tokio::main]
async fn main() {
    let matches = App::new("AI api server")
        .version("0.1")
        .author("Yuanhang Sun")
        .about("AI api server")
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
    info!("Bind to: {}", bind);
    info!("Debug mode: {}", debug);

    db::init(&CONFIG).await;

    // GET /, return a page with a form to upload a string
    let index = warp::path::end().map(|| {
        warp::reply::html(
            r#"
            <html>
                <head>
                    <meta charset="UTF-8">
                    <title>软件设计实践 API token 获取</title>
                </head>
                <body>
                    <h1>软件设计实践 API token 获取</h1>
                    <form action="/api/v1/ai/token" method="post">
                        <label for="id">学号</label>
                        <input type="text" id="id" name="id" required />
                        <input type="submit" value="提交" />
                    </form>
                </body>
            </html>
        "#,
        )
    });

    // POST /api/v1/ai/token, return a token
    let get_token_api = warp::path!("api" / "v1" / "ai" / "token")
        .and(warp::post())
        .and(warp::body::form())
        .then(handler::get_token);

    // POST /api/ai/v1/translate
    let translate_api = warp::path!("api" / "v1" / "ai" / "translate")
        .and(warp::post())
        .and(warp::header("Authorization"))
        .and(warp::body::json())
        .then(handler::translate);

    // /api/ai/v1/chat
    let chat_api = warp::path!("api" / "v1" / "ai" / "chat")
        .and(warp::post())
        .and(warp::header("Authorization"))
        .and(warp::body::json())
        .then(handler::chat);

    let draw_api = warp::path!("api" / "v1" / "ai" / "draw")
        .and(warp::post())
        .and(warp::header("Authorization"))
        .and(warp::body::json())
        .then(handler::draw);

    let math_api = warp::path!("api" / "v1" / "ai" / "wolfram")
        .and(warp::post())
        .and(warp::header("Authorization"))
        .and(warp::body::json())
        .then(handler::wolfram);

    // use reqwest to get google for conectivity test
    let client = reqwest::Client::new();
    let res = client.get("https://www.google.com").send().await.unwrap();
    let status = res.status();
    if status.is_success() {
        info!("Google connectivity test passed");
    } else {
        error!("Google connectivity test failed");
        return;
    }

    let routes = index
        .or(get_token_api)
        .or(translate_api)
        .or(chat_api)
        .or(draw_api)
        .or(math_api);

    warp::serve(routes).run(([0, 0, 0, 0], 4399)).await;
}
