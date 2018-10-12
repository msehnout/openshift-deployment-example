#[macro_use]
extern crate warp;
#[macro_use]
extern crate log;
extern crate simple_logger;

use warp::Filter;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    // GET /alive => 200 OK with body "yes"
    let hello = path!("alive").map(|| {
        info!("I am alive");
        "yes"
    });

    let f = path!("f" / u32)
        .and_then(|arg: u32| -> Result<String, warp::reject::Rejection> {
            Ok(format!("{}", arg*2))
        });

    let api = f.or(hello);

    let routes = api.with(warp::log(""));

    info!("Starting the server");
    warp::serve(routes).run(([0, 0, 0, 0], 8000));
}


