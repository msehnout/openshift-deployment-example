#[macro_use]
extern crate warp;
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate reqwest;

use warp::Filter;
use warp::Future;
use reqwest::async::Client;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let client = warp::any().map(move || Client::new());

    // GET /alive => 200 OK with body "yes"
    let hello = path!("alive")
        .map(|| {
            info!("I am alive");
            "yes"
        });

    let f = path!("f" / String)
        .and(client)
        .and_then(|arg, client: Client| {
            info!("spawn request");
            client
                .get(&format!("http://{}", arg)).send()
                .map_err(|e| {
                    error!("{}", e);
                    warp::reject::server_error()
                })
        })
        .map(|response| format!("res: {:#?}", response));

    let api = f.or(hello);

    let routes = api.with(warp::log(""));

    info!("Starting the server");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080));
}
