#[macro_use]
extern crate warp;
#[macro_use]
extern crate log;
extern crate simple_logger;

use warp::Filter;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    // GET /alive => 200 OK with body "yes"
    let hello = path!("alive")
        .map(|| {
            info!("I am alive");
            "yes"
        });

    info!("Starting the server");
    warp::serve(hello)
        .run(([0, 0, 0, 0], 8080));
}
