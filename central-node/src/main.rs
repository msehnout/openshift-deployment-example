#[macro_use]
extern crate warp;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate simple_logger;

use reqwest::async::{Client, Response};
use warp::Filter;
use warp::Future;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let client = warp::any().map(move || Client::new());

    // GET /alive => 200 OK with body "yes"
    let hello = path!("alive").map(|| {
        info!("I am alive");
        "yes"
    });

    let f = path!("f" / u32)
        .and(client)
        .and_then(|arg: u32, client: Client| {
            info!("spawn request");
            client.get(&format!("http://127.0.0.1:8000/f/{}", arg)).send().map_err(|e| {
                error!("{}", e);
                warp::reject::server_error()
            })
        }).map(|response: Response| format!("res: {:#?}", response));

    let api = f.or(hello);

    let routes = api.with(warp::log(""));

    info!("Starting the server");
    warp::serve(routes).run(([0, 0, 0, 0], 8080));
}
