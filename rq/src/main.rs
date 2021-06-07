// #![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate prometheus;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use prometheus::{IntCounter, IntCounterVec};
use warp::Filter;

mod handlers;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

lazy_static! {
    static ref PLAYGROUND_REQUESTS_TOTAL: IntCounter = register_int_counter!(opts!(
        "playground_requests_total",
        "Total number of HTTP requests made to /playground.",
        labels! {"handler" => "playground",}
    ))
    .unwrap();
    static ref INDEX_REQUESTS_TOTAL: IntCounter = register_int_counter!(opts!(
        "index_requests_total",
        "Total number of HTTP requests made to /.",
        labels! {"handler" => "index",}
    ))
    .unwrap();
    static ref REQUESTS_TOTAL: IntCounter = register_int_counter!(opts!(
        "requests_total",
        "Total number of HTTP requests made to app on all routes except /metrics",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    static ref RESPONSE_CODES: IntCounterVec = register_int_counter_vec!(
        opts!(
            "response_codes",
            "Response codes vec from app",
            labels! {"handler" => "response codes",}
        ),
        &["statusCode", "type"]
    )
    .unwrap();
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let index = warp::path::end().and(warp::get()).and_then(handlers::index);

    let health = warp::path("health")
        .and(warp::get())
        .and_then(handlers::health);

    let version = warp::path("version")
        .and(warp::get())
        .and_then(handlers::version);

    let playground = warp::path("playground")
        .and(warp::get())
        .and_then(handlers::playground);

    let metrics = warp::path("metrics")
        .and(warp::get())
        .and_then(handlers::metrics);

    let bad = warp::path("bad").and(warp::get()).and_then(handlers::bad);

    let routes = index
        .or(health)
        .or(metrics)
        .or(playground)
        .or(version)
        .or(bad);

    info!("rq starting: {}", VERSION);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
